use std::any::TypeId;

use bevy::{prelude::*, scene::serde::SceneDeserializer, utils::HashMap};

use game_state::prelude::*;
use save::prelude::*;
use serde::de::DeserializeSeed;

/// Normalizes the axes of a vector, so that each axes has a value of either 1.0 or 0.0.
fn normalize_axes(mut axes: Vec3) -> Vec3 {
    axes.x = if axes.x > 0. { 1. } else { 0. };
    axes.y = if axes.y > 0. { 1. } else { 0. };
    axes.z = if axes.z > 0. { 1. } else { 0. };
    axes
}

/// Utility that centers the position of all entities in a scene along the specified axes.
///
/// # Arguments:
///
/// * `scene` - The scene to operate on
/// * `axes` - The axes along which to center. Vec3::new(1.0, 1.0, 0.0) will only center objects along the X and Y axes.
///     If any of the axes are greater than 0.0, they will be set to 1.0.
pub fn center_entities_in_scene(scene: &mut Scene, mut axes: Vec3) {
    axes = normalize_axes(axes);

    // define a query that only returns the transforms of physics bodies in the scene
    let mut query = scene
        .world
        .query_filtered::<&mut Transform, With<PhysicsBody>>();

    // calculate the center by averaging all positions in the scene world
    let mut center = Vec3::ZERO;
    let mut count = 0;
    for transform in query.iter(&scene.world) {
        center += transform.translation;
        count += 1;
    }
    // prevent divide-by-zero errors
    if count > 0 {
        center /= count as f32;
    }
    center *= axes;

    // update all transforms in the scene world
    let mut count: u32 = 0;
    for mut transform in query.iter_mut(&mut scene.world) {
        transform.translation -= center;
        count += 1;
    }

    info!("Centered {count} entities in scene");
}

/// Utility that adds a component to all entities in a scene
pub fn add_component_to_all_entities_in_scene<C: Bundle + Clone>(scene: &mut Scene, bundle: C) {
    let entities: Vec<Entity> = scene.world.iter_entities().map(|e| e.id()).collect();
    for e in entities.iter() {
        scene.world.entity_mut(*e).insert(bundle.clone());
    }
}

/// Utility that creates a `Scene` from a `DynamicScene`, usually so we can mutate the scene before serializing it or
/// applying it to the world.
pub fn dynamic_scene_to_scene(
    dynamic_scene: &DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<Scene, String> {
    let mut scene = Scene::from_dynamic_scene(dynamic_scene, type_registry)
        .map_err(|err| format!("{err:?}"))?;
    scene.world.insert_resource(type_registry.clone());
    Ok(scene)
}

/// Utility that returns a vector of all children in an entity's hierarchy.
pub fn recursively_get_children_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    // define a vector of all entities in the heirarchy, including `entity`.
    let mut result = vec![entity];

    // Reccursively add children for all entities in `result`.
    // IMPORTANT: Verify the children exist first. `DynamicSceneBuilder::extract_entities` may panic otherwise.
    // This is necessary because `Children` may contain non-existent entities after being de-serialized.
    let mut children_query = (*world).query::<&Children>();
    let mut current = 0;
    while current < result.len() {
        let entity = result[current];
        if let Ok(children) = children_query.get(world, entity) {
            for &child in children
                .iter()
                .filter(|child| world.get_entity(**child).is_some())
            {
                result.push(child);
            }
        }
        current += 1;
    }

    result
}

/// Utility that returns the top-most parent entity in a hierarchy.
pub fn get_highest_parent_from_world(world: &mut World, entity: Entity) -> Entity {
    let mut parent_query = (*world).query::<&Parent>();

    let mut current = entity;
    while let Ok(parent) = parent_query.get(world, current) {
        current = parent.get();
    }
    current
}

/// Get all entities in the hierarchy from an entity in a world.
pub fn get_hierarchy_entities_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    let parent = get_highest_parent_from_world(world, entity);
    recursively_get_children_from_world(world, parent)
}

/// Get all connected entities for an entity in a world.
pub fn get_entity_family_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    let mut connector_query = (*world).query::<&HasConnector>();

    // TODO: Make this more efficient. We should not traverse the same hierarchy twice.
    let mut entities = get_hierarchy_entities_from_world(world, entity);
    let mut count = 0;
    while count < entities.len() {
        let entity = entities[count];
        let connected = connector_query
            .get(world, entity)
            .map(|connector| connector.0.clone())
            .ok();
        if let Some(connected) = connected {
            for &entity in connected.iter() {
                let connector_hierarchy = get_hierarchy_entities_from_world(world, entity);
                for entity in connector_hierarchy {
                    if !entities.contains(&entity) {
                        entities.push(entity);
                    }
                }
            }
        }
        count += 1;
    }

    entities
        .iter()
        .copied()
        .filter(|&e| world.entity(e).contains::<Saveable>())
        .collect()
}

/// Create a `DynamicScene` from an entity in a world.
///
/// # Panics
///
/// Panics if the world does not contain `AppTypeRegistry` or `SaveableRegistry` resources.
pub fn saveable_scene_from_entity(
    world: &mut World,
    entity: Entity,
    include_family: bool,
    include_children_components: bool,
) -> DynamicScene {
    // get all children
    let entities_to_save = if include_family {
        get_entity_family_from_world(world, entity)
    } else {
        // NOTE: Does not check for presence of `Saveable` component
        vec![entity]
    };
    info!("creating scene from {} entities", entities_to_save.len());

    // define a scene filter which only includes types registered in the `SaveableRegistry`
    let mut filter = get_saveable_scene_filter_from_world(world);

    // Optionally include children components, which are not registered as saveable by default, but that we may want
    // to include when capturing objects.
    // TODO: Maybe remove this, I can't remember if the issue was fixed. It was only needed when capturing objects.
    if include_children_components {
        filter = filter.allow_by_id(TypeId::of::<Children>());
    }

    // build the scene
    DynamicSceneBuilder::from_world(world)
        .with_filter(filter)
        .extract_entities(entities_to_save.into_iter())
        .remove_empty_entities()
        .build()
}

/// Deserialize a `DynamicScene` from a `String`.
pub fn deserialize_scene(
    serialized_scene: &str,
    type_registry: &AppTypeRegistry,
) -> Result<DynamicScene, String> {
    let mut deserializer =
        ron::de::Deserializer::from_str(serialized_scene).map_err(|err| format!("{err:?}"))?;
    let scene_deserializer = SceneDeserializer {
        type_registry: &type_registry.0.read(),
    };
    scene_deserializer
        .deserialize(&mut deserializer)
        .map_err(|err| format!("{err:?}"))
}

/// Creates a new scene from a dynamic scene asset in the world.
pub fn scene_from_world_dynamic_scene(
    scene_handle: impl Into<AssetId<DynamicScene>>,
    world: &mut World,
) -> Result<Scene, String> {
    // read the dynamic scene
    let assets = world
        .get_resource::<Assets<DynamicScene>>()
        .expect("World does not have an Assets<DynamicScene>");
    let dynamic_scene = assets
        .get(scene_handle)
        .ok_or("DynamicScene does not exist")?;

    // convert the `DynamicScene` to a `Scene`
    let type_registry = world.resource::<AppTypeRegistry>();
    dynamic_scene_to_scene(dynamic_scene, type_registry)
}

/// Utility that writes a dynamic scene to the world, and accepts a closure than gives mutable world acccess to each
/// spawned entity.
///
/// # Examples:
///
/// ```ignore
/// // Apply a transform and add a `Saveable` component to all spawned entities.
/// write_dynamic_scene_to_world(&dynamic_scene, world, |mut entity_mut| {
///     entity_mut.insert(Saveable);
///     if let Some(mut transform) = entity_mut.get_mut::<Transform>() {
///         *transform = new_transform.mul_transform(*transform);
///     }
/// });
/// ```
pub fn write_dynamic_scene_to_world(
    dynamic_scene: &DynamicScene,
    world: &mut World,
    update_fn: impl Fn(&mut EntityWorldMut<'_>),
) -> Result<(), String> {
    let mut entity_map = HashMap::<Entity, Entity>::new();
    dynamic_scene
        .write_to_world(world, &mut entity_map)
        .map_err(|err| format!("{err:?}"))?;

    // Call `update_fn` for each entity that was spawned
    for entity in entity_map.values() {
        if let Some(mut entity_mut) = world.get_entity_mut(*entity) {
            update_fn(&mut entity_mut);
        }
    }

    Ok(())
}

/// Utility that writes a dynamic scene asset to the world from it's handle, and accepts a closure than gives mutable
/// world acccess to each spawned entity.
///
/// Same as `write_dynamic_scene_to_world`, except it reads the `DynamicScene` asset first.
pub fn write_dynamic_scene_asset_to_world(
    dynamic_scene_id: impl Into<AssetId<DynamicScene>>,
    world: &mut World,
    update_fn: impl Fn(&mut EntityWorldMut<'_>),
) -> Result<(), String> {
    world.resource_scope(|world, scenes: Mut<Assets<DynamicScene>>| {
        let dynamic_scene = scenes
            .get(dynamic_scene_id)
            .ok_or("Scene asset not found".to_string())?;
        write_dynamic_scene_to_world(dynamic_scene, world, update_fn)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Component, Reflect, Default)]
    #[reflect(Component)]
    struct Foo(i32);

    #[test]
    fn deserialize_scene_works() {
        // define a type registry and register the types that will be in the scene
        let type_registry = AppTypeRegistry::default();
        {
            let mut registry = type_registry.write();
            registry.register::<Foo>();
        }

        // define a dynamic scene string
        let input = r#"(
  resources: {},
  entities: {
    0: (
      components: {
        "bevy_scene_utils::utils::tests::Foo": (123),
      },
    ),
  },
)"#;

        // deserialize the scene and test that it worked
        let scene = deserialize_scene(input, &type_registry).unwrap();
        assert_eq!(
            0,
            scene.resources.len(),
            "expected `resources` to contain 0 resource"
        );
        assert_eq!(
            1,
            scene.entities.len(),
            "expected `entities` to contain 1 entities"
        );
    }

    #[test]
    fn normalize_axes_works() {
        // Test already normalized values
        assert_eq!(normalize_axes(Vec3::X), Vec3::X);
        assert_eq!(normalize_axes(Vec3::Y), Vec3::Y);
        assert_eq!(normalize_axes(Vec3::Y), Vec3::Y);
        assert_eq!(normalize_axes(Vec3::ONE), Vec3::ONE);
        assert_eq!(normalize_axes(Vec3::ZERO), Vec3::ZERO);

        // Test non-normalized small values
        assert_eq!(normalize_axes(Vec3::splat(0.1)), Vec3::ONE);
        assert_eq!(normalize_axes(Vec3::new(0.1, 0., 0.)), Vec3::X);
        assert_eq!(normalize_axes(Vec3::new(0., 0.1, 0.)), Vec3::Y);
        assert_eq!(normalize_axes(Vec3::new(0., 0., 0.1)), Vec3::Z);

        // Test non-normalized large values
        assert_eq!(normalize_axes(Vec3::splat(2.)), Vec3::ONE);
        assert_eq!(normalize_axes(Vec3::new(2., 0., 0.)), Vec3::X);
        assert_eq!(normalize_axes(Vec3::new(0., 2., 0.)), Vec3::Y);
        assert_eq!(normalize_axes(Vec3::new(0., 0., 2.)), Vec3::Z);
    }
}

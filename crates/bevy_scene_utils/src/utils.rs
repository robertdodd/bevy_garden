use std::any::TypeId;

use bevy::{prelude::*, scene::serde::SceneDeserializer};

use game_state::prelude::*;
use save::prelude::*;
use serde::de::DeserializeSeed;

pub fn center_entities_in_scene(scene: &mut Scene) {
    let mut query = scene
        .world
        .query_filtered::<&mut Transform, With<PhysicsBody>>();

    // calculate the average position in the scene world so we can use it to center the entities
    let center = {
        let mut center = Vec2::ZERO;
        let mut translation_count = 0;
        for transform in query.iter(&scene.world) {
            center += transform.translation.truncate();
            translation_count += 1;
        }
        center / translation_count as f32
    };

    // update all transforms in the scene world
    let mut count: u32 = 0;
    for mut transform in query.iter_mut(&mut scene.world) {
        transform.translation -= center.extend(0.0);
        count += 1;
    }

    info!("Centered {count} entities in scene");
}

pub fn add_component_to_all_entities_in_scene<C: Bundle + Clone>(scene: &mut Scene, bundle: C) {
    // let mut query = scene.world.query_filtered::<Entity, Without<C>>();
    // for e in query.iter_mut(&mut scene.world) {
    let entities: Vec<Entity> = scene.world.iter_entities().map(|e| e.id()).collect();
    for e in entities.iter() {
        scene.world.entity_mut(*e).insert(bundle.clone());
    }
}

/// Create a `Scene` from a `DynamicScene`, usually so we can mutate the scene before serializing/applying it to the
/// world.
pub fn dynamic_scene_to_scene(
    dynamic_scene: &DynamicScene,
    type_registry: &AppTypeRegistry,
) -> Result<Scene, String> {
    let mut scene = Scene::from_dynamic_scene(dynamic_scene, type_registry)
        .map_err(|err| format!("{err:?}"))?;
    scene.world.insert_resource(type_registry.clone());

    Ok(scene)
}

/// Create a `DynamicScene` from a `Scene`
pub fn scene_to_dynamic_scene(scene: &Scene) -> DynamicScene {
    DynamicScene::from_scene(scene)
}

/// get all children belonging to an entity from a world
pub fn recursively_get_children_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    let mut children_query = (*world).query::<&Children>();

    // define the result vector, and add `entity` to it. It should contain at least one entity before starting the
    // while loop below.
    let mut result = Vec::<Entity>::new();
    result.push(entity);

    // loop through `result`, adding children for each entity until we reach the end
    let mut current = 0;
    while current < result.len() {
        let entity = result[current];
        if let Ok(children) = children_query.get(world, entity) {
            for &child in children.iter() {
                // Verify the entity exists. `DynamicSceneBuilder::extract_entities` will panic if one of the entities
                // does not exist. This is necessary because `Children` may contain non-existent entities after being
                // de-serialized.
                if world.get_entity(child).is_some() {
                    result.push(child);
                }
            }
        }
        current += 1;
    }

    result
}

/// get the top-most parent entity for an entity in a world
pub fn get_highest_parent_from_world(world: &mut World, entity: Entity) -> Entity {
    let mut parent_query = (*world).query::<&Parent>();

    let mut current = entity;
    while let Ok(parent) = parent_query.get(world, current) {
        current = parent.get();
    }
    current
}

/// Get all entities in the heirarchy from an entity in a world.
pub fn get_heirarchy_entities_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    let parent = get_highest_parent_from_world(world, entity);
    recursively_get_children_from_world(world, parent)
}

/// Get all connected entities for an entity in a world.
pub fn get_entity_family_from_world(world: &mut World, entity: Entity) -> Vec<Entity> {
    let mut connector_query = (*world).query::<&HasConnector>();

    // TODO: Make this more efficient. We should not traverse the same heirarchy twice.
    let mut entities = get_heirarchy_entities_from_world(world, entity);
    let mut count = 0;
    while count < entities.len() {
        let entity = entities[count];
        let connected = connector_query
            .get(world, entity)
            .map(|connector| connector.0.clone())
            .ok();
        if let Some(connected) = connected {
            for &entity in connected.iter() {
                let connector_heirarchy = get_heirarchy_entities_from_world(world, entity);
                for entity in connector_heirarchy {
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

    // NOTE: panic if either resource does not exist
    let type_registry = world.resource::<AppTypeRegistry>();
    let saveable = world.resource::<SaveableRegistry>();

    // define a scene filter which only includes types registered in the `SaveableRegistry`
    let mut filter = SceneFilter::deny_all();
    for type_registration in type_registry.read().iter() {
        if saveable.contains(type_registration.type_info().type_path()) {
            filter = filter.allow_by_id(type_registration.type_id());
        }
    }

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
}

use bevy::{ecs::system::Command, prelude::*};

use crate::*;

/// Command that manipulates a scene before spawning a `DynamicSceneBundle` as a child of some entity.
///
/// The scene is cloned and manipulated first. The `scene_marker` component is added to all entities in the scene.
/// Use this to add a `Disabled` marker to the scene when used by a tool.
///
/// * `scene`: Handle to the scene you want to spawn.
/// * `entity`: The entity the `dynamicSceneBundle` will be spawned as a child of.
/// * `child_bundle`: Components to add to the `DynamicSceneBundle` entity.
/// * `scene_marker`: Marker component that will be added to all entities in the scene before spawning.
#[derive(Debug)]
pub struct ExtractSceneToChildCommand<B: Bundle, C: Bundle + Clone> {
    pub scene: Handle<DynamicScene>,
    pub entity: Entity,
    pub child_bundle: B,
    pub scene_bundle: C,
}

impl<B, C> Command for ExtractSceneToChildCommand<B, C>
where
    for<'de> B: Bundle,
    for<'de> C: Bundle + Clone,
{
    fn apply(self, world: &mut World) {
        // read the dynamic scene
        let assets = world
            .get_resource::<Assets<DynamicScene>>()
            .expect("World does not have an Assets<DynamicScene>");
        let dynamic_scene = assets.get(&self.scene).unwrap();

        // convert from `DynamicScene` to `Scene`
        let type_registry = world.resource::<AppTypeRegistry>();
        let mut scene = dynamic_scene_to_scene(dynamic_scene, type_registry)
            .expect("Failed to create scene from dynamic scene");

        // Disable all entities in the scene
        add_component_to_all_entities_in_scene(&mut scene, self.scene_bundle);

        // Create a new scene asset
        let mut assets = world
            .get_resource_mut::<Assets<DynamicScene>>()
            .expect("World does not have an Assets<DynamicScene>");
        let dynamic_scene_handle = assets.add(scene_to_dynamic_scene(&scene));

        // spawn the new scene as a child of the tool entity
        info!(
            "ExtractSceneToChildCommand ==> spawning scene as child of {:?}",
            self.entity
        );
        let mut tool = world.entity_mut(self.entity);
        tool.with_children(|p| {
            p.spawn((
                self.child_bundle,
                DynamicSceneBundle {
                    scene: dynamic_scene_handle,
                    ..default()
                },
            ));
        });
    }
}

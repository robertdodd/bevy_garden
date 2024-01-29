use bevy::{ecs::system::Command, prelude::*};

use crate::*;

/// Command that manipulates a scene before spawning a `DynamicSceneBundle` as a child of some entity.
///
/// The scene is cloned and manipulated first. The `scene_marker` component is added to all entities in the scene.
/// Use this to add a `Disabled` marker to the scene when used by a tool.
///
/// # Arguments:
///
/// * `scene`: Handle to the scene you want to spawn.
/// * `entity`: The entity the `dynamicSceneBundle` will be spawned as a child of.
/// * `child_bundle`: Bundle added to the `DynamicSceneBundle` entity.
/// * `scene_bundle`: Bundle added to all entities in the scene before spawning.
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
        // Create a new scene from the dynamic scene
        let mut scene = scene_from_world_dynamic_scene(&self.scene, world)
            .expect("Failed to get scene from world");

        // Add components to all entities in the scene.
        // This is usually used to add `Disabled` components to everything before spawning it.
        add_component_to_all_entities_in_scene(&mut scene, self.scene_bundle);

        // Create a new scene asset
        let mut assets = world
            .get_resource_mut::<Assets<DynamicScene>>()
            .expect("World does not have an Assets<DynamicScene>");
        let dynamic_scene_handle = assets.add(DynamicScene::from_scene(&scene));

        // spawn the new scene as a child of the tool entity
        info!(
            "ExtractSceneToChildCommand ==> spawning scene as child of {:?}",
            self.entity
        );
        world.entity_mut(self.entity).with_children(|p| {
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

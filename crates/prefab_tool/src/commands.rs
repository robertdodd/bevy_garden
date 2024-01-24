use bevy::{ecs::system::Command, prelude::*, utils::HashMap};

use bevy_scene_utils::dynamic_scene_to_scene;
use save::prelude::*;

use crate::types::PrefabToolResult;

/// Command that writes a scene to the world without keeping a reference to the scene like `SceneSpawner` does.
///
/// This allows us to tell asset server to reload the asset without the side effects of `SceneSpawner` updating
/// existing entities spawned from that scene
pub(crate) struct SpawnPrefabCommand {
    pub tool_name: String,
    pub scene_handle: Handle<DynamicScene>,
    pub transform: Transform,
    pub parent: Option<Entity>,
}

impl Command for SpawnPrefabCommand {
    fn apply(self, world: &mut World) {
        let result: Result<DynamicScene, String> =
            world.resource_scope(|world, scenes: Mut<Assets<DynamicScene>>| {
                let dynamic_scene = scenes
                    .get(&self.scene_handle)
                    .map_or(Err("Could not load scene".to_string()), Ok)?;

                // Create a `Scene` from the `DynamicScene` so that we can transform it applying to the world
                let type_registry = world.resource::<AppTypeRegistry>();
                let mut scene = dynamic_scene_to_scene(dynamic_scene, type_registry)
                    .map_err(|err| format!("{err:?}"))?;

                // Mark all entities as saveable so that it can be saved right away when we send the RollbackSave event
                // at the end of this command.
                let entities: Vec<Entity> = scene.world.iter_entities().map(|e| e.id()).collect();
                for entity in entities.iter() {
                    if let Some(mut entity_mut) = scene.world.get_entity_mut(*entity) {
                        entity_mut.insert(Saveable);
                    }
                }

                // apply transform to scene world
                let mut query = scene
                    .world
                    .query_filtered::<&mut Transform, Without<Parent>>();
                for mut transform in query.iter_mut(&mut scene.world) {
                    *transform = self.transform;
                }

                // NOTE: if using `rapier` physics, you might want to clear any `Velocity` components in the scene

                Ok(DynamicScene::from_scene(&scene))
            });

        match result {
            Ok(dynamic_scene) => {
                // Write the scene to the world
                let mut entity_map = HashMap::<Entity, Entity>::new();
                dynamic_scene
                    .write_to_world(world, &mut entity_map)
                    .map_err(|err| format!("{err:?}"))
                    .expect("failed");

                // handle reparenting, reparent all entities that were spawned by getting them from the entity map
                if let Some(parent) = self.parent {
                    for entity in entity_map.values() {
                        if let Some(mut entity_mut) = world.get_entity_mut(*entity) {
                            if entity_mut.get::<Parent>().is_none() {
                                entity_mut.set_parent(parent);
                            }
                        }
                    }
                }

                // send success event
                world.send_event(PrefabToolResult(Ok(self.tool_name.clone())));
            }
            Err(err) => {
                // send error event
                world.send_event(PrefabToolResult(Err(format!(
                    "Error spawning {}: {}",
                    self.tool_name, err
                ))));
            }
        }
    }
}

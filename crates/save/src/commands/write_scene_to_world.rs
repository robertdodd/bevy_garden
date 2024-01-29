use bevy::{ecs::system::Command, prelude::*, utils::HashMap};

use crate::events::SaveResult;

/// Writes a scene to the world without keeping a reference to the scene.
/// This differs from `SceneSpawner`, which does keep a reference to the scene asset. By not keeping a reference to
/// the the scene asset, we can reload the asset without the side effects of `SceneSpawner` updating existing entities
/// spawned from that scene.
#[derive(Debug)]
pub(crate) struct WriteSceneToWorldCommand {
    pub(crate) scene_handle: Handle<DynamicScene>,
}

impl Command for WriteSceneToWorldCommand {
    fn apply(self, world: &mut World) {
        info!("[Save] ==> applying WriteSceneToWorldCommand");

        // Write the dynamic scene to the world.
        let result: Result<(), String> =
            world.resource_scope(|world, scenes: Mut<Assets<DynamicScene>>| {
                // read the dynamic scene asset
                let dynamic_scene = scenes
                    .get(&self.scene_handle)
                    .ok_or("DynamicScene asset does not exist".to_string())?;

                // write the dynamic scene to the world
                let mut entity_map = HashMap::<Entity, Entity>::default();
                dynamic_scene
                    .write_to_world(world, &mut entity_map)
                    .map_err(|err| format!("{err}"))?;

                // Re-parent all new entities that have a `Parent` component. This is required because we do not save
                // `Children` components, given that they may contain entity IDs that were not saved.
                for entity in entity_map.values() {
                    // Check whether the entity has a parent component, and that the parent exists.
                    let parent = world
                        .get_entity(*entity)
                        .and_then(|entity| entity.get::<Parent>())
                        .map(|parent| parent.get())
                        // Check that the parent actually exists in the world
                        .filter(|parent| world.get_entity(*parent).is_some());

                    // if the entity has a parent, then parent it to that entity
                    // TODO: Should we remove the `Parent` component if the parent does not exist?
                    if let Some(parent) = parent {
                        if let Some(mut entity) = world.get_entity_mut(*entity) {
                            entity.set_parent(parent);
                        }
                    }
                }

                Ok(())
            });

        // emit the success/fail event
        if let Err(err) = result {
            error!("Error applying WriteSceneToWorldCommand: {err:?}");
            world.send_event(SaveResult::LevelLoad(Err(format!("{err:?}"))));
        } else {
            info!("successfully loaded world");
            world.send_event(SaveResult::LevelLoad(Ok(())));
        }
    }
}

use bevy::{ecs::system::Command, prelude::*, scene::SceneSpawnError, utils::HashMap};

use crate::events::SaveResult;

/// Writes a scene to the world without keeping a reference to the scene.
/// This differs from `SceneSpawner`, which does keep a reference to the scene asset. This command allows us to reload
/// the asset without the side effects of `SceneSpawner` updating existing entities spawned from that scene.
#[derive(Debug)]
pub(crate) struct WriteSceneToWorldCommand {
    pub(crate) scene_handle: Handle<DynamicScene>,
}

impl Command for WriteSceneToWorldCommand {
    fn apply(self, world: &mut World) {
        info!("[Save] ==> applying WriteSceneToWorldCommand");

        // Write the scene to the world and return the new `Entity` IDs, which we can get from the `EntityMap` after
        // the scene was applied. We use the entity IDs to re-parent any with a `Parent` component.
        let result = world.resource_scope(|world, scenes: Mut<Assets<DynamicScene>>| {
            // load the scene
            let scene = scenes.get(&self.scene_handle).ok_or_else(|| {
                SceneSpawnError::NonExistentScene {
                    id: self.scene_handle.id(),
                }
            })?;
            // write the scene to the world
            let mut entity_map = HashMap::<Entity, Entity>::default();
            scene
                .write_to_world(world, &mut entity_map)
                .map(|_| entity_map.values().cloned().collect::<Vec<Entity>>())
        });

        // Re-parent all loaded entities with a `Parent` component. This is required because we do not save
        // `Children` components, given that they may contain entity IDs that were not saved.
        if let Ok(ref entities) = result {
            for entity in entities.iter() {
                // Check whether the entity has a parent component, and that the parent exists.
                let parent = world
                    .get_entity(*entity)
                    .and_then(|entity| entity.get::<Parent>())
                    .map(|parent| parent.get());
                // Check that the parent exists
                let parent = parent.filter(|parent| world.get_entity(*parent).is_some());

                // if the entity has a parent, then parent it to that entity
                if let Some(parent) = parent {
                    if let Some(mut entity) = world.get_entity_mut(*entity) {
                        entity.set_parent(parent);
                    }
                }
            }
        }

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

use bevy::{ecs::system::Command, prelude::*};

use bevy_scene_utils::write_dynamic_scene_asset_to_world;
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
        let result = write_dynamic_scene_asset_to_world(self.scene_handle, world, |entity_mut| {
            // IMPORTANT: Mark all entities as saveable so they can be saved right away. We send a `RollbackSave` event at
            // the end of the command, so if the entities aren't marked as saveable they won't be saved.
            entity_mut.insert(Saveable);

            // apply transform to any entities that do not have a parent
            let has_parent = entity_mut.contains::<Parent>();
            if !has_parent {
                if let Some(mut entity_transform) = entity_mut.get_mut::<Transform>() {
                    *entity_transform = self.transform.mul_transform(*entity_transform);
                }
            }

            // NOTE: if using a physics engine, you might want to clear any `Velocity` components in the scene
            // if let Some(mut velocity) = entity_mut.get_mut::<Velocity>() {
            //     velocity.angvel = 0.;
            //     velocity.linvel = Vec2::ZERO;
            // }

            // Set parent on any entities that don't have a parent
            if let Some(parent) = self.parent {
                if entity_mut.get::<Parent>().is_none() {
                    entity_mut.set_parent(parent);
                }
            }
        });

        // Send events with the result so we can handle the result in the tool plugin
        match result {
            Ok(_) => {
                world.send_event(PrefabToolResult(Ok(self.tool_name.clone())));
            }
            Err(err) => {
                world.send_event(PrefabToolResult(Err(format!(
                    "Error spawning {}: {}",
                    self.tool_name, err
                ))));
            }
        }
    }
}

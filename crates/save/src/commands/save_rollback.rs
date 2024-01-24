use bevy::{ecs::system::Command, prelude::*};

use crate::{events::SaveResult, rollbacks::Rollbacks};

use super::utils::*;

/// Command that saves a rollback checkpoint of the world.
#[derive(Debug)]
pub(crate) struct SaveRollbackCommand;

impl Command for SaveRollbackCommand {
    fn apply(self, world: &mut World) {
        info!("[Save] ==> applying SaveRollbackCommand");

        // create the scene from the current world
        let scene = saveable_scene_from_world(world);

        // add the scene directly to the asset collection
        let mut assets = world.resource_mut::<Assets<DynamicScene>>();
        let scene_handle = assets.add(scene);

        // push the scene handle into the rollbacks vector
        let mut rollbacks = world.resource_mut::<Rollbacks>();
        rollbacks.push_checkpoint(scene_handle);

        info!(
            "[Save] ==> Rollback saved. Current: {:?}, Total: {:?}",
            rollbacks.active,
            rollbacks.count()
        );

        // emit the success result
        emit_save_result_event(world, SaveResult::RollbackSave(Ok(())));
    }
}

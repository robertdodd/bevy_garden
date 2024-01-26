use bevy::{ecs::system::Command, prelude::*};

#[cfg(not(target_arch = "wasm32"))]
use {
    super::utils::*,
    crate::events::SaveResult,
    bevy::tasks::IoTaskPool,
    std::{fs::File, io::Write},
};

/// Command that saves the level to a file.
///
/// * `filename` - Filename relative the the assets folder. NOTE: do not include the "assets/" prefix.
#[derive(Debug)]
pub(crate) struct SaveLevelCommand {
    pub(crate) filename: String,
}

#[cfg(not(target_arch = "wasm32"))]
impl Command for SaveLevelCommand {
    fn apply(self, world: &mut World) {
        info!("[Save] ==> applying SaveLevelCommand");

        let filename = format!("assets/{}", self.filename);

        // ensure directory exists
        if let Err(err) = ensure_directory_exists_for_filename(&filename) {
            error!(err);
            world.send_event(SaveResult::LevelSave(Err(err)));
            return;
        }

        // create the scene
        let scene = saveable_scene_from_world(world);

        // serialize the scene so we can save it
        let type_registry = world.resource::<AppTypeRegistry>();
        let serialized_scene = scene
            .serialize_ron(type_registry)
            .map_err(|err| format!("{err:?}"));
        if let Err(err) = serialized_scene {
            error!("error serializing scene: {err}");
            world.send_event(SaveResult::LevelSave(Err(err)));
            return;
        }
        let serialized_scene = serialized_scene.unwrap();

        // Writing the scene to a new file. Using a task to avoid calling the filesystem APIs in a system
        // as they are blocking
        IoTaskPool::get()
            .spawn(async move {
                // Write the scene RON data to file
                File::create(filename)
                    .and_then(|mut file| file.write(serialized_scene.as_bytes()))
                    .expect("Error writing scene to file");
            })
            .detach();

        // emit the success event
        // TODO: Can we listen for the `IoTaskPool` result, and include error messages?
        world.send_event(SaveResult::LevelSave(Ok(self.filename)));
    }
}

#[cfg(target_arch = "wasm32")]
/// An implementation of the save level command for WASM, which does nothing currently because we need file system
/// access.
/// TODO: implement this using LocalStorage
impl Command for SaveLevelCommand {
    fn apply(self, _world: &mut World) {
        info!(
            "[Save] ==> ignoring SaveLevelCommand on wasm target: assets/{}",
            self.filename
        );
    }
}

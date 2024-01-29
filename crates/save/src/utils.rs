use std::{fs::create_dir_all, path::Path};

use bevy::prelude::*;

use crate::registry::SaveableRegistry;

// This function is public, so it can be conveniently used by the client.
/// Creates the full directory path to a filename.
///
/// # Arguments:
///
/// * `filename` - Path to a file. This must be the path to a file and not to a folder, because it only creates
///     directories for the "parent" path of `filename`.
pub fn ensure_directory_exists_for_filename(filename: &String) -> Result<(), String> {
    let prefix = Path::new(filename.as_str())
        .parent()
        .ok_or(format!("error extracting prefix from path: {filename}"))?;
    create_dir_all(prefix).map_err(|_| format!("error creating directory path: {filename}"))
}

/// Create a `SceneFilter` that only includes components registered in the world's `SaveableRegistry`.
///
/// # Panics
///
/// Panics if the world does not contain `AppTypeRegistry` or `SaveableRegistry` resources.
pub fn get_saveable_scene_filter_from_world(world: &mut World) -> SceneFilter {
    let type_registry = world.resource::<AppTypeRegistry>();
    let saveable = world.resource::<SaveableRegistry>();

    // define a scene filter which only includes types registered in the `SaveableRegistry`
    let mut filter = SceneFilter::deny_all();
    for type_registration in type_registry.read().iter() {
        if saveable.contains(type_registration.type_info().type_path()) {
            filter = filter.allow_by_id(type_registration.type_id());
        }
    }

    filter
}

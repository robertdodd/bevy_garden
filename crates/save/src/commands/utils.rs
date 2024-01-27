use std::{fs::create_dir_all, path::Path};

use bevy::prelude::*;

use crate::{registry::SaveableRegistry, rollbacks::Rollbacks, types::*};

use super::WriteSceneToWorldCommand;

// Despawn all entities with `Saveable` or `DespawnOnLoad` components.
#[allow(clippy::type_complexity)]
pub(crate) fn despawn_saveable_entities(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// Applies a scene handle to the world
#[allow(clippy::type_complexity)]
pub(crate) fn apply_scene_handle(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    scene_handle: Handle<DynamicScene>,
) {
    despawn_saveable_entities(commands, query);
    let cmd = WriteSceneToWorldCommand {
        scene_handle: scene_handle.clone(),
    };
    commands.add(cmd);
}

/// Applies a rollback in the specified direction
#[allow(clippy::type_complexity)]
pub(crate) fn apply_rollback(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    rollbacks: &mut ResMut<Rollbacks>,
    direction: isize,
) -> Result<(), ()> {
    if !rollbacks.has_checkpoint(direction) {
        return Err(());
    }
    if let Some(scene_handle) = rollbacks.rollback(direction) {
        apply_scene_handle(commands, query, scene_handle);
        Ok(())
    } else {
        Err(())
    }
}

// This function is public, so it can be conveniently used by the client.
/// Creates the full directory path to a filename.
///
/// * `filename` - Path to a file, relative to the "assets" directory. This must be the path to a file and not to a
///     folder, because it only creates directories for the prefix of `filename`.
pub fn ensure_directory_exists_for_filename(filename: &String) -> Result<(), String> {
    // get the prefix (i.e. folder path)
    let path = Path::new(filename.as_str());
    let prefix = path
        .parent()
        .ok_or(format!("error extracting prefix from path: {filename}"))?;

    // create the full directory path
    create_dir_all(prefix).map_err(|_| format!("error creating directory path: {filename}"))
}

/// Extracts a `DynamicScene` from the world.
pub(crate) fn saveable_scene_from_world(world: &mut World) -> DynamicScene {
    // extract the entities we want to save
    let mut query = world.query_filtered::<Entity, With<Saveable>>();
    let entities: Vec<Entity> = query.iter(world).collect();

    // get a scene filter from the world that only includes types registered in the `SaveableRegistry`
    let filter = get_saveable_scene_filter_from_world(world);

    // build the scene
    DynamicSceneBuilder::from_world(world)
        .with_filter(filter)
        .extract_entities(entities.into_iter())
        .remove_empty_entities()
        .build()
}

/// Create a `SceneFilter` that only includes components registered in the world's `SaveableRegistry`.
pub fn get_saveable_scene_filter_from_world(world: &mut World) -> SceneFilter {
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

    filter
}

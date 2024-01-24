use std::{fs::create_dir_all, path::Path};

use bevy::{ecs::system::SystemState, prelude::*};

use crate::{events::*, registry::SaveableRegistry, rollbacks::Rollbacks, types::*};

use super::WriteSceneToWorldCommand;

#[allow(clippy::type_complexity)]
pub(crate) fn despawn_saveable_entities(
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
) {
    // Despawn all saveable/despawn-on-load entities
    // There is a bug in Bevy where despawning sounds does not stop them playing, so we need to stop those sounds
    // before despawning them. I think we can do them in the same loop, because the despawn command is only applied
    // at the end of the frame.
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

/// Emits a save result event from the world. For use by commands.
pub(crate) fn emit_save_result_event(world: &mut World, event: SaveResult) {
    let mut save_events_system_state = SystemState::<ResMut<Events<SaveResult>>>::new(world);
    let mut save_events = save_events_system_state.get_mut(world);
    save_events.send(event);
}

// This function is public so it can be conveniently used by the client.
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

    // build the scene
    DynamicSceneBuilder::from_world(world)
        .with_filter(filter)
        .extract_entities(entities.into_iter())
        .remove_empty_entities()
        .build()
}

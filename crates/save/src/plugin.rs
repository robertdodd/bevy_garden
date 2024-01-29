use bevy::{asset::LoadState, prelude::*};

use super::{app::AppSaveableExt, commands::*, events::*, rollbacks::Rollbacks, types::*};

/// Plugin that adds saving and loading to an app.
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Rollbacks::default())
            .add_event::<RollbackSaveEvent>()
            .add_event::<LevelLoadSuccess>()
            .add_event::<LevelLoadFail>()
            .add_event::<SaveEvent>()
            .add_event::<LoadEvent>()
            .add_event::<RollbackBackEvent>()
            .add_event::<RollbackLoadEvent>()
            .add_event::<RollbackForwardEvent>()
            .add_event::<RollbackClearEvent>()
            .add_event::<SaveResult>()
            // Register our types as inspect-able
            .register_type::<Saveable>()
            .register_type::<DespawnOnLoad>()
            // Register core types as saveable
            .register_saveable::<Parent>()
            // NOTE: Children components to be excluded because they may contain the IDs of entities which were not
            // saved. `WriteSceneToWorldCommand` MUST re-parent the entities after applying the scene.
            .register_saveable::<Transform>()
            .register_saveable::<Name>()
            // Run systems after the update set. Not sure if this is best?
            .add_systems(
                PostUpdate,
                (
                    handle_rollback_save_events.run_if(on_event::<RollbackSaveEvent>()),
                    handle_rollback_back_events.run_if(on_event::<RollbackBackEvent>()),
                    handle_rollback_load_events.run_if(on_event::<RollbackLoadEvent>()),
                    handle_rollback_forward_events.run_if(on_event::<RollbackForwardEvent>()),
                    handle_rollback_clear_events.run_if(on_event::<RollbackClearEvent>()),
                    handle_save_events.run_if(on_event::<SaveEvent>()),
                    handle_load_events.run_if(on_event::<LoadEvent>()),
                    handle_pending_levels.run_if(resource_exists::<PendingLevelLoad>()),
                ),
            );
    }
}

/// Saves a new rollback when it receives a `RollbackSaveEvent`
fn handle_rollback_save_events(
    mut commands: Commands,
    mut rollback_events: EventReader<RollbackSaveEvent>,
) {
    for _ in rollback_events.read() {
        commands.add(SaveRollbackCommand);
    }
}

#[allow(clippy::type_complexity)]
fn handle_rollback_load_events(
    mut commands: Commands,
    mut events: EventReader<RollbackLoadEvent>,
    query: Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    mut rollbacks: ResMut<Rollbacks>,
    mut success_writer: EventWriter<LevelLoadSuccess>,
    mut fail_writer: EventWriter<LevelLoadFail>,
) {
    for _ in events.read() {
        let result = apply_rollback(&mut commands, &query, &mut rollbacks, 0);
        // TODO: Fix result event messages/path
        match result {
            Ok(_) => success_writer.send(LevelLoadSuccess("".to_string())),
            Err(_) => {
                error!("No more rollbacks");
                fail_writer.send(LevelLoadFail {
                    path: "".to_string(),
                    error: "".to_string(),
                });
            }
        }
    }
}

/// Applies the previous rollback (Undo action)
#[allow(clippy::type_complexity)]
fn handle_rollback_back_events(
    mut commands: Commands,
    mut rollback_events: EventReader<RollbackBackEvent>,
    query: Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    mut rollbacks: ResMut<Rollbacks>,
    mut save_result_writer: EventWriter<SaveResult>,
) {
    for _ in rollback_events.read() {
        apply_rollback_event(
            1,
            &mut commands,
            &query,
            &mut rollbacks,
            &mut save_result_writer,
        );
    }
}

/// Applies the next rollback (Redo action)
#[allow(clippy::type_complexity)]
fn handle_rollback_forward_events(
    mut commands: Commands,
    mut rollback_events: EventReader<RollbackForwardEvent>,
    query: Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    mut rollbacks: ResMut<Rollbacks>,
    mut save_result_writer: EventWriter<SaveResult>,
) {
    for _ in rollback_events.read() {
        apply_rollback_event(
            -1,
            &mut commands,
            &query,
            &mut rollbacks,
            &mut save_result_writer,
        );
    }
}

/// Utility to apply a rollback and handle the result. Used by the systems that handle rollback events.
#[allow(clippy::type_complexity)]
fn apply_rollback_event(
    direction: isize,
    commands: &mut Commands,
    query: &Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    rollbacks: &mut ResMut<Rollbacks>,
    save_result_writer: &mut EventWriter<SaveResult>,
) {
    let result = apply_rollback(commands, query, rollbacks, direction);
    if result.is_ok() {
        save_result_writer.send(SaveResult::RollbackApply(Ok(())))
    } else {
        error!("No more rollbacks");
        save_result_writer.send(SaveResult::RollbackApply(Err(
            "No more rollbacks".to_string()
        )))
    }
}

/// Clears the rollback list when receiving a `RollbackClearEvent`
fn handle_rollback_clear_events(
    mut clear_events: EventReader<RollbackClearEvent>,
    mut rollbacks: ResMut<Rollbacks>,
) {
    for _ in clear_events.read() {
        rollbacks.clear_checkpoints();
    }
}

/// Saves the level when it receives a `SaveEvent`
fn handle_save_events(mut commands: Commands, mut save_events: EventReader<SaveEvent>) {
    for event in save_events.read() {
        let cmd = SaveLevelCommand {
            filename: event.filename.clone(),
        };
        commands.add(cmd);
    }
}

/// Loads a level from a file when it receives a `LoadEvent`
#[allow(clippy::type_complexity)]
fn handle_load_events(
    mut commands: Commands,
    mut load_events: EventReader<LoadEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in load_events.read() {
        // tell the asset server to reload the asset, so we get a fresh copy in case it has been changed recently
        asset_server.reload(event.filename.clone());

        // insert a `PendingLevelLoad` resource, which will apply the level to the world after it has finished loading
        commands.insert_resource(PendingLevelLoad {
            path: event.filename.clone(),
            handle: asset_server.load(event.filename.clone()),
        });
    }
}

/// System that waits for a level asset to finish loading before writing it to the world.
#[allow(clippy::type_complexity)]
fn handle_pending_levels(
    mut commands: Commands,
    pending_level: Res<PendingLevelLoad>,
    asset_server: Res<AssetServer>,
    query: Query<Entity, Or<(With<DespawnOnLoad>, With<Saveable>)>>,
    mut success_events: EventWriter<LevelLoadSuccess>,
    mut fail_events: EventWriter<LevelLoadFail>,
) {
    let load_state = asset_server.get_load_state(pending_level.handle.clone());
    match load_state {
        Some(LoadState::Loaded) => {
            info!("Pending level loaded: {:?}", pending_level.path);

            // Despawn all saveable/despawn-on-load entities
            despawn_saveable_entities(&mut commands, &query);

            // Write the scene to world
            let cmd = WriteSceneToWorldCommand {
                scene_handle: pending_level.handle.clone(),
            };
            commands.add(cmd);

            // Send the success event, and remove the pending level load resource
            success_events.send(LevelLoadSuccess(pending_level.path.clone()));
            commands.remove_resource::<PendingLevelLoad>();
        }
        Some(LoadState::Failed) => {
            error!("Failed to load level: {:?}", pending_level.path);
            fail_events.send(LevelLoadFail {
                path: pending_level.path.clone(),
                error: "Failed to load the level".to_string(),
            });
            commands.remove_resource::<PendingLevelLoad>();
        }
        _ => {
            info!(
                "Pending level in {:?} state: {:?}",
                load_state, pending_level.handle
            );
        }
    }
}

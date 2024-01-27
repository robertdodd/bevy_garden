use bevy::prelude::*;
use bevy_asset_loader::prelude::{LoadingState, LoadingStateAppExt};

use bevy_helpers::generic_systems::{debug_state_changes, despawn_recursive_with};

use crate::prelude::*;

/// Handles systems and state transitions while in `PlayState`.
///
/// Play state is initiated by the user transitioning to `GameState::Game`.
///
/// State transitions for `PlayState` happen in the following order:
///
/// - AssetLoading
///   - Initiated by `OnEnter(GameState::Game)`.
///   - Loads external assets. Other plugins should add assets to the loading state via the `bevy_asset_loader`
///     package.
///   - Transition to next state happens automatically via `bevy_asset_loader`.
/// - SetupAssets
///   - Initiated by `bevy_asset_loader` once assets have loaded.
///   - Handles setting up assets after they have loaded, e.g. populating `PhysicsMaterials` resource
///   - Immediately transitions to the next state.
/// - SetupResources
///   - Initiated by `OnEnter(PlayState::SetupAssets)``
///   - Handles setting up internal resources, e.g. `init_resource` calls that finish in the same frame.
///   - Immediately transitions to the next state.
/// - Loading
///   - Handled manually, usually to load the level.
///   - Initiated by `OnEnter(PlayState::SetupResources)`
///   - Responsible for loading the current level if applicable, or setting it up.
///   - Should transition to `PlayState::SetupLevel` if successful, `PlayState::FailedToLoad` otherwise. Can also skip
///   - directly to `PlayState::Play` or `PlayState::Pause`.
/// - SetupLevel
///   - Handled manually.
///   - Initiated manually by app, so can be skipped entirely.
///   - Only the `PlayerPlugin` depends on it.
///   - Responsible for setting up the current level.
/// - FailedToLoad (optional)
///   - Initiated manually.
///   - Handled manually.
pub(crate) struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
            .add_state::<PlayState>()
            .add_state::<PauseState>()
            .add_event::<PauseEvent>()
            .add_loading_state(
                LoadingState::new(PlayState::LoadAssets)
                    .continue_to_state(PlayState::SetupAssets)
                    .on_failure_continue_to_state(PlayState::FailedToLoad),
            )
            .add_systems(
                OnEnter(GameState::Game),
                transition_to_play_asset_loading.run_if(play_state_is_none),
            )
            .add_systems(
                OnEnter(PlayState::SetupAssets),
                transition_to_play_setup_resources,
            )
            .add_systems(
                OnEnter(PlayState::SetupResources),
                transition_to_play_loading,
            )
            .add_systems(OnEnter(PlayState::SetupLevel), transition_to_play_active)
            .add_systems(OnEnter(PlayState::Active), un_pause_game_on_play)
            .add_systems(
                OnExit(GameState::Game),
                (
                    transition_to_play_none_when_leaving_game,
                    despawn_recursive_with::<GameCamera>,
                    pause_when_leaving_game,
                ),
            )
            .add_systems(
                Update,
                (
                    debug_state_changes::<GameState>,
                    debug_state_changes::<PlayState>,
                    handle_pause_events
                        .run_if(on_event::<PauseEvent>())
                        .run_if(in_state(GameState::Game)),
                    (despawn_game_entities, update_despawn_state)
                        .run_if(in_state(GameState::ExitGame)),
                    despawn_game_entities.run_if(in_state(GameState::ReloadLevel)),
                    handle_reload_level_complete.run_if(in_state(GameState::ReloadLevel)),
                ),
            );
    }
}

/// Maximum number of entities to despawn per frame when de-spawning the level
const DESPAWN_COUNT_PER_FRAME: usize = 10;

/// System which reads `PauseEvent` events and toggles `PauseState`.
fn handle_pause_events(
    mut events: EventReader<PauseEvent>,
    mut pause_state: ResMut<NextState<PauseState>>,
) {
    for event in events.read() {
        pause_state.set(event.0);
    }
}

/// System which de-spawns entities in small batches each frame. This prevents the game freezing on a single frame when
/// de-spawning a large level.
fn despawn_game_entities(mut commands: Commands, query: Query<Entity, With<GameMarker>>) {
    let mut count: usize = 0;
    for entity in query.iter() {
        count += 1;
        commands.entity(entity).despawn_recursive();
        if count >= DESPAWN_COUNT_PER_FRAME {
            break;
        }
    }
}

/// System which runs in `GameState::ExitGame`. It waits until all game entities have finished de-spawning before
/// transitioning to `GameState::Menu`.
fn update_despawn_state(
    query: Query<Entity, With<GameMarker>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if query.is_empty() {
        game_state.set(GameState::Menu);
    }
}

/// System which transitions to `PauseState::Paused` when leaving the game.
fn pause_when_leaving_game(mut pause_state: ResMut<NextState<PauseState>>) {
    pause_state.set(PauseState::Pause);
}

/// System which disables `PlayState` when leaving the game.
fn transition_to_play_none_when_leaving_game(mut next_play_state: ResMut<NextState<PlayState>>) {
    next_play_state.set(PlayState::None);
}

/// System which instantly transitions to `PlayState::LoadAssets`. All system in the current state should finish in
/// a single frame.
fn transition_to_play_asset_loading(mut next_play_state: ResMut<NextState<PlayState>>) {
    next_play_state.set(PlayState::LoadAssets);
}

/// System which runs in `GameState::ReloadLevel`. It waits until all game entities have finished de-spawning before
/// transitioning to `GameState::Game` and transitioning to the first stage of `PlayState`.
fn handle_reload_level_complete(
    query: Query<Entity, With<GameMarker>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut next_play_state: ResMut<NextState<PlayState>>,
) {
    if query.is_empty() {
        game_state.set(GameState::Game);
        next_play_state.set(PlayState::LoadAssets);
    }
}

/// System which instantly transitions to `PlayState::SetupResource`. All system in the current state should finish in
/// a single frame.
fn transition_to_play_setup_resources(mut next_play_state: ResMut<NextState<PlayState>>) {
    next_play_state.set(PlayState::SetupResources);
}

/// System which instantly transitions to `PlayState::LoadLevel`. All system in the current state should finish in
/// a single frame.
fn transition_to_play_loading(mut next_play_state: ResMut<NextState<PlayState>>) {
    next_play_state.set(PlayState::LoadLevel);
}

/// System which instantly transitions to `PlayState::Active`. All system in the current state should finish in
/// a single frame.
fn transition_to_play_active(mut next_play_state: ResMut<NextState<PlayState>>) {
    next_play_state.set(PlayState::Active);
}

/// System which un-pauses the game when all the setup states have finished and `PlayState::Active` is entered.
fn un_pause_game_on_play(mut pause_state: ResMut<NextState<PauseState>>) {
    pause_state.set(PauseState::Play);
}

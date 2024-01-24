use bevy::prelude::*;

use super::{PauseState, PlayState};

/// Runs a system when in game and not paused
pub fn not_paused(play_state: Res<State<PlayState>>, pause_state: Res<State<PauseState>>) -> bool {
    *play_state.get() == PlayState::Active && *pause_state.get() == PauseState::Play
}

/// Runs a system when the game is active and during the level setup states. Does not include the asset/resource
/// loading states.
pub fn in_game(play_state: Res<State<PlayState>>) -> bool {
    match play_state.get() {
        // true:
        PlayState::LoadLevel => true,
        PlayState::SetupLevel => true,
        PlayState::Active => true,
        // false:
        _ => false,
    }
}

/// Runs a system when in `PlayState::None` state
pub fn play_state_is_none(play_state: Res<State<PlayState>>) -> bool {
    *play_state.get() == PlayState::None
}

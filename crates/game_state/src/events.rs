use bevy::prelude::*;

use crate::state::PauseState;

/// Event used to pause/un-pause the game.
#[derive(Event)]
pub struct PauseEvent(pub PauseState);

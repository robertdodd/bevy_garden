use bevy::prelude::*;

use save::prelude::*;

use crate::{game_camera::GameCameraPlugin, prelude::*, state::plugin::StatePlugin};

/// Plugin which handles shared components, state and system sets for the game.
pub struct GameStatePlugin;

impl Plugin for GameStatePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            (
                SetupSet::RigidBody.run_if(in_game),
                SetupSet::Joint.run_if(in_game),
                SetupSet::Attachable.run_if(in_game),
            )
                .chain(),
        )
        .add_plugins((StatePlugin, GameCameraPlugin))
        // Disabled must be registered so that we can add it to scenes before spawning. E.g. scene tools. It should
        // not be saved though.
        .register_type::<Disabled>()
        // PhysicsBody must be registered as saveable so that we can correctly identify and center objects in scenes
        .register_saveable::<PhysicsBody>();
    }
}

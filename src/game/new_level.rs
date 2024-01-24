use bevy::prelude::*;

use background::prelude::*;
use game_state::prelude::*;
use save::prelude::*;
use tree::prelude::TreeBundle;

pub struct NewLevelPlugin;

impl Plugin for NewLevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::LoadLevel), spawn_new_level);
    }
}

/// Spawn the initial new level.
/// NOTE: if we were saving/loading from a file we would load and spawn the level here instead, and use a resource
/// that specifies whether to load from file or create a new level.
#[allow(clippy::type_complexity)]
fn spawn_new_level(
    mut commands: Commands,
    mut next_state: ResMut<NextState<PlayState>>,
    mut rollback_save_writer: EventWriter<RollbackSaveEvent>,
) {
    // Spawn a background
    commands.spawn(BackgroundBundle {
        background: Background {
            background_color: Color::BLUE,
            ground_color: Color::GREEN,
            size: 20.,
            ambient_light: 0.25,
            directional_light: 32_000.0,
        },
        saveable: Saveable,
        despawn_on_load: DespawnOnLoad,
    });

    // Spawn a tree
    commands.spawn(TreeBundle::new(Transform::from_translation(Vec3::ZERO)));

    // Transition to the next state
    next_state.set(PlayState::SetupLevel);

    // Save the initial rollback
    rollback_save_writer.send(RollbackSaveEvent);
}

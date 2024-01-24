use bevy::prelude::*;

use crate::prelude::*;

/// Plugin which manages cameras in game.
///
/// It automatically spawns a GameCamera when you enter the game.
///
/// While this example does not include it, this is where you would handle the following:
/// - Set up camera depending on graphics settings (anti-aliasing, bloom, tone-mapping)
/// - Handle camera movement (velocity, following players etc.)
/// - Handle camera boundaries to prevent the camera from leaving the sccene.
pub struct GameCameraPlugin;

impl Plugin for GameCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::SetupLevel), setup_camera_on_enter_game);
    }
}

fn setup_camera_on_enter_game(mut commands: Commands) {
    commands.spawn((
        GameMarker,
        GameCamera,
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 10.0)
                .looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
            ..default()
        },
    ));
}

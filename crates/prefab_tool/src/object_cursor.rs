use bevy::prelude::*;

use editor::prelude::*;
use game_state::prelude::*;

use crate::types::PrefabToolCursorSet;

/// Plugin which handles cursors that place objects on the ground.
pub struct ObjectCursorPlugin;

impl Plugin for ObjectCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceObjectEvent>().add_systems(
            Update,
            (update_cursor_position, handle_mouse_click)
                .chain()
                .in_set(PrefabToolCursorSet)
                .run_if(in_game)
                .run_if(cursor_not_blocked),
        );
    }
}

/// A cursor that places objects on the ground
#[derive(Component)]
pub(crate) struct ObjectCursor;

/// Event emitted by this plugin when the mouse is clicked in a valid position
#[derive(Event)]
pub(crate) struct PlaceObjectEvent {
    pub tool: Entity,
    pub transform: Transform,
}

/// System that updates the cursor transform
fn update_cursor_position(
    mut query: Query<&mut Transform, With<ObjectCursor>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    windows: Query<&Window>,
) {
    for mut transform in query.iter_mut() {
        let camera_and_cursor = windows
            .get_single()
            .ok()
            .and_then(|window| window.cursor_position())
            .and_then(|cursor_pos| {
                camera_query
                    .get_single()
                    .ok()
                    .map(|(camera, camera_transform)| (camera, camera_transform, cursor_pos))
            });

        let new_translation =
            camera_and_cursor.and_then(|(camera, camera_transform, cursor_pos)| {
                camera
                    .viewport_to_world(camera_transform, cursor_pos)
                    .and_then(|ray| {
                        ray.intersect_plane(Vec3::ZERO, Vec3::Y)
                            .map(|distance| ray.get_point(distance))
                    })
            });

        if let Some(new_translation) = new_translation {
            transform.translation = new_translation;
        }
    }
}

/// System that emits `PlaceObjectEvent`s when the mouse is clicked
fn handle_mouse_click(
    query: Query<(Entity, &Transform), With<ObjectCursor>>,
    buttons: Res<Input<MouseButton>>,
    mut place_writer: EventWriter<PlaceObjectEvent>,
) {
    for (entity, transform) in query.iter() {
        if buttons.just_pressed(MouseButton::Left) {
            info!("[Object Cursor] ==> Clicked, emitting PlaceObjectEvent");
            place_writer.send(PlaceObjectEvent {
                tool: entity,
                transform: *transform,
            });
        }
    }
}

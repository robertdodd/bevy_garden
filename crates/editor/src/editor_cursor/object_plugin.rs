use bevy::prelude::*;

use game_state::prelude::*;

use crate::prelude::get_camera_and_cursor_pos;

use super::{cursor_not_blocked, EditorCursorSet};

/// Plugin which handles cursors that place objects on the ground.
///
/// The `Transform` of `ObjectCursor`s is updated each frame, and `PlaceObjectEvent` events are emitted when the mouse
/// is clicked.
pub(crate) struct ObjectCursorPlugin;

impl Plugin for ObjectCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceObjectEvent>().add_systems(
            Update,
            (
                update_cursor_position.in_set(EditorCursorSet::Transform),
                handle_mouse_click.in_set(EditorCursorSet::Click),
            )
                .run_if(in_game)
                .run_if(cursor_not_blocked),
        );
    }
}

/// A cursor used for placing objects on the ground
#[derive(Component)]
pub struct ObjectCursor;

/// Event emitted by this plugin when the mouse is clicked
#[derive(Event)]
pub struct PlaceObjectEvent {
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
        let camera_and_cursor = get_camera_and_cursor_pos(&windows, &camera_query);
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

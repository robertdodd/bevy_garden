use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use game_state::prelude::*;

use crate::prelude::pick_entity_from_raycast;

use super::{cursor_not_blocked, EditorCursorSet};

/// Plugin which handles cursors that place attachables.
///
/// It is responsible for updating the cursor `Transform` and sending `PlaceAttachableEvent` when the mouse is clicked.
pub(crate) struct AttachableCursorPlugin;

impl Plugin for AttachableCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<PlaceAttachableEvent>().add_systems(
            Update,
            (
                setup_new_cursors.run_if(in_game),
                (
                    update_cursor_position.in_set(EditorCursorSet::Transform),
                    handle_mouse_click.in_set(EditorCursorSet::Click),
                )
                    .chain()
                    .run_if(cursor_not_blocked),
            ),
        );
    }
}

/// Distance in front of the camera to display the cursor when nothing is picked by the raycast.
const DEFAULT_PLANE_DISTANCE: f32 = 10.;

/// Cursor that places objects as children on other entities
#[derive(Component)]
pub struct AttachableCursor {
    pub distance: f32,
    pub forward: Vec3,
}

/// Internal component for tracking the raycast hit data
#[derive(Component, Default)]
struct AttachableCursorTarget(Option<HitData>);

/// Internal type describing the raycast hit data
#[derive(Debug)]
struct HitData {
    entity: Entity,
    world_pos: Vec3,
    world_normal: Vec3,
    local_anchor: Vec3,
    local_normal: Vec3,
}

/// Event emitted by this plugin when the mouse is clicked on a valid target
#[derive(Event)]
pub struct PlaceAttachableEvent {
    pub tool: Entity,
    pub target: Entity,
    pub transform: Transform,
}

/// System that sets up newly added `AttachableCursor` entities
fn setup_new_cursors(mut commands: Commands, query: Query<Entity, Added<AttachableCursor>>) {
    for entity in query.iter() {
        commands
            .entity(entity)
            .insert(AttachableCursorTarget::default());
    }
}

/// System that runs raycasts and updates the cursor position every frame.
#[allow(clippy::too_many_arguments)]
fn update_cursor_position(
    cursor_ray: Res<CursorRay>,
    mut raycast: Raycast,
    mut query: Query<(
        &AttachableCursor,
        &mut AttachableCursorTarget,
        &mut Transform,
    )>,
    target_query: Query<&GlobalTransform, With<AcceptsAttachables>>,
    family_child_query: Query<&FamilyChild>,
    pickable_query: Query<(), With<Pickable>>,
    camera_query: Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    windows: Query<&Window>,
) {
    for (cursor, mut cursor_target, mut transform) in query.iter_mut() {
        // Perform the raycast
        let result = pick_entity_from_raycast(
            &cursor_ray,
            &mut raycast,
            &pickable_query,
            &family_child_query,
        );

        // Map the intersection to a `HitData` type by adding local coordinates to the result
        let hit_data = result.and_then(|(hit_entity, hit_data)| {
            // Get the parent entity
            let target_entity = family_child_query
                .get(hit_entity)
                .map_or(hit_entity, |family_child| family_child.0);

            // Get the world coordinates by offsetting along the normal by the `cursor.distance`, making sure to scale
            // the distance.
            let world_normal = hit_data.normal();
            let world_pos =
                hit_data.position() + (world_normal * cursor.distance * transform.scale.x);

            // Transform the intersection to local coordinates
            let local_coords = target_query
                .get(target_entity)
                .map(|transform| {
                    let inverse_affine = transform.compute_transform().compute_affine().inverse();
                    let local_anchor = inverse_affine.transform_point3(world_pos);
                    let local_normal = inverse_affine.transform_vector3(world_normal);
                    (local_anchor, local_normal)
                })
                .ok();

            local_coords.map(|(local_anchor, local_normal)| HitData {
                entity: target_entity,
                world_pos,
                world_normal,
                local_anchor,
                local_normal,
            })
        });

        // Update cursor transform.
        // If the raycast was successful, use the hit data. If not, project the cursor position to a fixed distance
        // in front of the camera.
        if let Some(hit_data) = hit_data.as_ref() {
            *transform = Transform::from_translation(hit_data.world_pos)
                .with_rotation(Quat::from_rotation_arc(
                    cursor.forward,
                    hit_data.world_normal.normalize(),
                ))
                .with_scale(transform.scale);
        } else {
            // If there are no collisions, then get the cursor world position at the (0.0, 0.0) plane
            let new_transform = get_non_intersecting_transform(
                &camera_query,
                &windows,
                transform.scale,
                cursor.forward,
            );
            if let Some(new_transform) = new_transform {
                *transform = new_transform;
            }
        }

        // Update cursor target
        cursor_target.0 = hit_data;
    }
}

/// Utility that projects the cursor to a point a fixed distance in front of the camera.
/// This ensures that the cursor transform still follows the mouse. It should appear that the object you're placing is
/// floating in the air a short distance in front of the camera.
#[allow(clippy::too_many_arguments)]
fn get_non_intersecting_transform(
    camera_query: &Query<(&Camera, &GlobalTransform), With<GameCamera>>,
    windows: &Query<&Window>,
    scale: Vec3,
    forward: Vec3,
) -> Option<Transform> {
    let camera_and_cursor_pos = windows
        .get_single()
        .ok()
        .and_then(|window| window.cursor_position())
        .and_then(|cursor_pos| {
            camera_query
                .get_single()
                .ok()
                .map(|(camera, camera_transform)| (camera, camera_transform, cursor_pos))
        });

    // STYLE: We could string this together with above, but it saves an indentation level like this
    camera_and_cursor_pos.and_then(|(camera, camera_transform, cursor_pos)| {
        // define the plane at a certain distance in front of the camera. We'll intersect that plane with the cursor
        // to get the resulting transform.
        let camera_forward = camera_transform.forward();
        let plane_origin = camera_transform.translation() + camera_forward * DEFAULT_PLANE_DISTANCE;
        let plane_normal = -camera_forward;

        camera
            .viewport_to_world(camera_transform, cursor_pos)
            .and_then(|ray| {
                ray.intersect_plane(plane_origin, plane_normal)
                    .map(|distance| ray.get_point(distance))
            })
            .map(|translation| {
                Transform::from_translation(translation)
                    .with_rotation(Quat::from_rotation_arc(forward, -camera_forward))
                    .with_scale(scale)
            })
    })
}

/// System that emits events to place the object when the mouse button is clicked.
fn handle_mouse_click(
    query: Query<(
        Entity,
        &AttachableCursor,
        &Transform,
        &AttachableCursorTarget,
    )>,
    buttons: Res<Input<MouseButton>>,
    mut place_writer: EventWriter<PlaceAttachableEvent>,
    target_query: Query<&GlobalTransform>,
) {
    // STYLE: This saves one level of indentation
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }

    for (entity, cursor, cursor_transform, cursor_target) in query.iter() {
        let target_transform_and_hit_data = cursor_target.0.as_ref().and_then(|hit_data| {
            target_query
                .get(hit_data.entity)
                .ok()
                .map(|target_transform| (target_transform, hit_data))
        });

        if let Some((target_transform, hit_data)) = target_transform_and_hit_data {
            place_writer.send(PlaceAttachableEvent {
                tool: entity,
                // Convert transform to local coordinates. Once we place the object it will be parented to target.
                transform: cursor_transform
                    .with_translation(hit_data.local_anchor)
                    .with_rotation(Quat::from_rotation_arc(
                        cursor.forward,
                        hit_data.local_normal,
                    ))
                    .with_scale(
                        cursor_transform.scale / target_transform.compute_transform().scale,
                    ),
                target: hit_data.entity,
            });
        }
    }
}

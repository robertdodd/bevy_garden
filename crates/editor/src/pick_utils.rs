use bevy::prelude::*;
use bevy_mod_raycast::prelude::*;

use game_state::prelude::*;

/// Utility for picking an entity using a bevy_mod_raycast
pub fn pick_entity_from_raycast(
    cursor_ray: &CursorRay,
    raycast: &mut Raycast,
    pickable_query: &Query<(), With<Pickable>>,
    family_child_query: &Query<&FamilyChild>,
) -> Option<(Entity, IntersectionData)> {
    if let Some(cursor_ray) = **cursor_ray {
        let settings = RaycastSettings {
            visibility: RaycastVisibility::MustBeVisibleAndInView,
            filter: &|entity| pickable_query.contains(entity),
            early_exit_test: &|_| true,
        };
        let hits = raycast.cast_ray(cursor_ray, &settings);

        hits.iter().next().map(|(hit_entity, hit_data)| {
            // get the parent entity from the hit
            let entity_parent = family_child_query
                .get(*hit_entity)
                .map_or(*hit_entity, |family_child| family_child.0);
            (entity_parent, hit_data.clone())
        })
    } else {
        None
    }
}

pub fn get_camera_and_cursor_pos<'a>(
    windows: &'a Query<&Window>,
    camera_query: &'a Query<(&Camera, &GlobalTransform), With<GameCamera>>,
) -> Option<(&'a Camera, &'a GlobalTransform, Vec2)> {
    windows
        .get_single()
        .ok()
        .and_then(|window| window.cursor_position())
        .and_then(|cursor_pos| {
            camera_query
                .get_single()
                .ok()
                .map(|(camera, camera_transform)| (camera, camera_transform, cursor_pos))
        })
}

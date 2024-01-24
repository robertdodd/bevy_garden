use bevy::prelude::*;

use save::prelude::*;

#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Background {
    pub background_color: Color,
    pub ground_color: Color,
    pub size: f32,
    pub ambient_light: f32,
    pub directional_light: f32,
}

#[derive(Bundle)]
pub struct BackgroundBundle {
    pub background: Background,
    pub saveable: Saveable,
    pub despawn_on_load: DespawnOnLoad,
}

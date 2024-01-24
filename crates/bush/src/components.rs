use bevy::prelude::*;

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bush;

#[derive(Component)]
pub struct BushMesh {
    pub local_anchor: Vec3,
}

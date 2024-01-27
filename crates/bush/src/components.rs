use bevy::prelude::*;

/// A bush.
///
/// Must be spawned with a `Transform` component.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Bush;

/// A mesh belonging to a bush.
///
/// It is anchored to the mesh by the `local_anchor` property, and its transform is updated every frame.
#[derive(Component)]
pub struct BushMesh {
    pub local_anchor: Vec3,
}

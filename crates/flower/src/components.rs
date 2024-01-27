use bevy::prelude::*;

/// A flower.
///
/// This component should always be spawned with a `Transform` component and as a child of another entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Flower;

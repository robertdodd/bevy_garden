use bevy::prelude::*;
use save::types::Saveable;

/// A tree.
///
/// Must be spawned with a `TreeSize` and `Transform` component.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Tree;

/// The size of a tree
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct TreeSize {
    pub trunk_thickness: f32,
    pub trunk_height: f32,
    pub canopy: Vec3,
}

/// Bundle for spawning a tree.
///
/// This represents what the scene files for the tree tools should contain.
///
/// This bundle is only used by the `new_level` module when spawning a new level. If it wasn't used there then this
/// type should be deleted.
#[derive(Bundle)]
pub struct TreeBundle {
    pub tree: Tree,
    pub size: TreeSize,
    pub transform: Transform,
    pub saveable: Saveable,
}

impl TreeBundle {
    pub fn new(transform: Transform) -> Self {
        Self {
            tree: Tree,
            size: TreeSize {
                trunk_thickness: 0.25,
                trunk_height: 2.0,
                canopy: Vec3::new(2., 1., 2.),
            },
            transform,
            saveable: Saveable,
        }
    }
}

/// NOTE: The reason we have a separate file only to hold a single type is for consistent type paths in bevy scene
/// files.
///
/// See the "prefab.scn.ron" asset files for examples, the `Apple` type path is "apple::components::Apple".
///
/// All (workspace crate) type paths follow the same convention:
/// "[crate_name]::components::[ComponentName]"
///
/// This can become important because if a type path changes, all scene files referencing it will break. It also makes
/// them predictable, allowing you to write a prefab asset without having to refer to exactly what the type paths are.
use bevy::prelude::*;

/// An apple.
///
/// This component should always be spawned with a `Transform` component and as a child of another entity.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Apple;

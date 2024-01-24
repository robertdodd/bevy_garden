use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct PointerTool;

#[derive(Component)]
pub(crate) struct PointerToolSelection(pub Option<Entity>);

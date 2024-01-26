use bevy::prelude::*;

#[derive(Component)]
pub(crate) struct DuplicateTool;

#[derive(Component, Clone)]
pub(crate) struct OnDuplicateTool;

#[derive(Event, Debug)]
pub(crate) struct CaptureObjectToSceneResult(pub Result<Handle<DynamicScene>, String>);

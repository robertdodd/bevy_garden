use bevy::prelude::*;

use crate::assets::ToolScaling;

/// The prefab tool cursor
#[derive(Component)]
pub(crate) struct PrefabToolCursor {
    pub name: String,
    pub scene_handle: Handle<DynamicScene>,
    pub scaling: Option<ToolScaling>,
}

/// Marker component for the despawning the tag tool
#[derive(Component, Clone)]
pub(crate) struct OnPrefabTool;

/// Marks the prefab tool cursor as finished loading the scene asset
#[derive(Component)]
pub(crate) struct PrefabToolOk;

/// Resource holding the path to the tool definition
#[derive(Resource)]
pub(crate) struct PrefabToolPath(pub String);

/// System set for crate cursor systems
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub(crate) struct PrefabToolCursorSet;

#[derive(Event)]
pub(crate) struct PrefabToolResult(pub Result<String, String>);

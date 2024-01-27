use bevy::prelude::*;

use editor::prelude::{PrefabConfig, ToolScaling};

/// The prefab tool cursor
#[derive(Component)]
pub(crate) struct PrefabToolCursor {
    pub name: String,
    pub scene_handle: Handle<DynamicScene>,
    pub scaling: Option<ToolScaling>,
}

/// Marker component for the de-spawning the tag tool
#[derive(Component, Clone)]
pub(crate) struct OnPrefabTool;

/// Marks the prefab tool cursor as finished loading the scene asset
#[derive(Component)]
pub(crate) struct PrefabToolOk;

/// Resource holding the path to the tool definition
#[derive(Resource)]
pub(crate) struct PrefabToolConfig(pub PrefabConfig);

#[derive(Event)]
pub(crate) struct PrefabToolResult(pub Result<String, String>);

use bevy::{prelude::*, reflect::TypePath};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ToolScaling {
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AttachableConfig {
    pub distance: f32,
    pub forward: Vec3,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PrefabToolType {
    Attachable(AttachableConfig),
    Object,
}

#[derive(Asset, Debug, Serialize, Deserialize, TypePath)]
pub struct PrefabTool {
    /// Name of the tool. Should be the "noun" describing the object the tool places.
    pub name: String,

    /// Unique key to identify the tool.
    pub key: String,

    // How the tool behaves, i.e. whether it is placing an attachable object, or a physics object.
    pub tool_type: PrefabToolType,

    /// Initial scale. Defaults to 1.0.
    #[serde(default = "PrefabTool::default_scale")]
    pub initial_scale: f32,

    /// Whether the prefab can be scaled before placing
    pub scaling: Option<ToolScaling>,
}

impl PrefabTool {
    pub fn default_scale() -> f32 {
        1.0
    }
}

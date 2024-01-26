use bevy::{prelude::*, reflect::TypePath};
use serde::{Deserialize, Serialize};

/// Configuration controlling the minimum and maximum an object can be scaled when placing.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrefabToolAssetScaling {
    /// Minimum scale amount
    pub min: f32,
    /// Maximum scale amount
    pub max: f32,
}

/// Configuration for attachable tools. Controls how they are oriented when placing.
#[derive(Debug, Serialize, Deserialize)]
pub struct PrefabToolAssetAttachable {
    /// Offset the object from the surface its placed on, in the direction of the surface normal.
    pub distance: f32,
    /// The forward direction of the object. Orients the object so this vector faces in the direction of the surface
    /// normal that its placed on.
    pub forward: Vec3,
}

/// The type of prefab tool, controlling how it is placed into the world.
#[derive(Debug, Serialize, Deserialize)]
pub enum PrefabToolAssetType {
    Attachable(PrefabToolAssetAttachable),
    Object,
}

#[derive(Asset, Debug, Serialize, Deserialize, TypePath)]
pub struct PrefabToolAsset {
    /// Name of the tool. Should be the object "noun".
    pub name: String,

    /// Unique key to identify the tool. The ToolLibrary will panic if you atttempt to add duplicate keys.
    pub key: String,

    /// How the tool behaves, can be either an attachable (anchored to a parent entity), or a physics object, which is
    /// placed on the ground.
    pub tool_type: PrefabToolAssetType,

    /// Initial scale. Defaults to 1.0.
    #[serde(default = "PrefabToolAsset::default_initial_scale")]
    pub initial_scale: f32,

    /// Optional min and max scaling values. A value of `None` will disable scaling.
    pub scaling: Option<PrefabToolAssetScaling>,
}

impl PrefabToolAsset {
    /// Default value for the `initial_scale` property
    pub fn default_initial_scale() -> f32 {
        1.0
    }
}

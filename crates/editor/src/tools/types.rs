use bevy::prelude::*;

/// All available tool types
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Tool {
    #[default]
    Pointer,
    Duplicate,
    Prefab(PrefabConfig),
    /// This tool should have nothing associated with it. This type is only activated when we exit the game/editor.
    None,
}

/// Event emitted when the current tool changes. It is emitted automatically when the tool stack resource changes.
#[derive(Event, Debug)]
pub struct ToolChangedEvent(pub Tool);

/// Event which pushes a new tool to the top of the stack
#[derive(Event, Debug)]
pub struct PushToolEvent(pub Tool);

/// Event which replaces the current tool in the stack
#[derive(Event, Debug)]
pub struct ReplaceToolEvent(pub Tool);

/// Event which cancels the current tool
#[derive(Event)]
pub struct PopToolEvent;

/// Marker component added to UI Nodes that blocks interactions from the mouse cursor from going through
#[derive(Component)]
pub struct UiFocusNode;

#[derive(Debug, PartialEq, Clone, TypePath)]
pub struct PrefabConfig {
    /// Name of the tool. Should be the object "noun".
    pub name: String,

    /// Handle to the scene
    pub scene: Handle<DynamicScene>,

    /// How the tool behaves, can be either an attachable (anchored to a parent entity), or a physics object, which is
    /// placed on the ground.
    pub tool_type: PrefabToolType,

    /// Initial scale.
    pub initial_scale: f32,

    /// Optional min and max scaling values. A value of `None` will disable scaling.
    pub scaling: Option<ToolScaling>,
}

/// Configuration for attachable tools. Controls how they are oriented when placing.
#[derive(Debug, PartialEq, Clone)]
pub struct PrefabAttachableConfig {
    /// Offset the object from the surface its placed on, in the direction of the surface normal.
    pub distance: f32,
    /// The forward direction of the object. Orients the object so this vector faces in the direction of the surface
    /// normal that its placed on.
    pub forward: Vec3,
}

/// The type of prefab tool, controlling how it is placed into the world.
#[derive(Debug, PartialEq, Clone)]
pub enum PrefabToolType {
    Attachable(PrefabAttachableConfig),
    Object,
}

/// Configuration controlling the minimum and maximum an object can be scaled when placing.
#[derive(Debug, Clone, PartialEq)]
pub struct ToolScaling {
    /// Minimum scale amount
    pub min: f32,
    /// Maximum scale amount
    pub max: f32,
}

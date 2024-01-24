use bevy::prelude::*;

/// All available tool types
#[derive(Default, Debug, Clone, PartialEq)]
pub enum Tool {
    #[default]
    Pointer,
    Prefab(String),
    // This tool should have nothing associated with it
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

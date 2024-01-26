use bevy::prelude::*;

#[derive(Resource, PartialEq, Eq, Clone, Copy, Debug, Default)]
pub enum EditorCursorStatus {
    #[default]
    Blocked,
    Active,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum EditorCursorSet {
    /// Update the cursor status to disable it when it is blocked by the UI.
    Status,
    /// Update the transform of cursor entities.
    Transform,
    /// Handle mouse click actions. This is where events to place the object are emitted. Any event handling systems
    /// should run AFTER this set.
    Click,
}

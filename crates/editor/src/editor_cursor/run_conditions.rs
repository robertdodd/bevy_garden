use bevy::prelude::*;

use super::EditorCursorStatus;

/// Run condition that returns true if the cursor is not blocked by a `PickableBlock` component.
pub fn cursor_not_blocked(cursor_status: Res<EditorCursorStatus>) -> bool {
    *cursor_status != EditorCursorStatus::Blocked
}

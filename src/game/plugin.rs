use bevy::prelude::*;

use game_state::prelude::*;
use save::prelude::*;

use crate::config::SAVE_FILENAME;

use super::{
    failed_to_load_menu::FailedToLoadMenuPlugin, new_level::NewLevelPlugin,
    tool_loader::ToolLoaderPlugin, tool_panel::ToolPanelPlugin,
};

/// Plugin responsible for in-game menus, ui, state transitions etc.
///
/// Basically, this plugin allows the player to interact with the game and triggers all the appropriate
/// state-transitions and events.
///
/// This is where you would add a main menu, settings menu, level-select screen, FPS counter, score UI etc.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ToolPanelPlugin,
            ToolLoaderPlugin,
            NewLevelPlugin,
            FailedToLoadMenuPlugin,
        ))
        .add_systems(Update, handle_keypress.run_if(in_state(PlayState::Active)));
    }
}

/// System that handles key presses while in the game, namely Undo/Redo.
fn handle_keypress(
    keys: Res<Input<KeyCode>>,
    mut roll_back_writer: EventWriter<RollbackBackEvent>,
    mut roll_forward_writer: EventWriter<RollbackForwardEvent>,
    mut save_writer: EventWriter<SaveEvent>,
    mut load_writer: EventWriter<LoadEvent>,
) {
    let is_control = keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight);
    let is_shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // Undo (Ctrl + Z)
    if is_control && !is_shift && keys.just_pressed(KeyCode::Z) {
        roll_back_writer.send(RollbackBackEvent);
    }

    // Redo (Ctrl + Y || Ctrl + Shift + Z)
    if is_control
        && ((is_shift && keys.just_pressed(KeyCode::Z))
            || (!is_shift && keys.just_pressed(KeyCode::Y)))
    {
        roll_forward_writer.send(RollbackForwardEvent);
    }

    // Save the level
    if is_control && !is_shift && keys.just_pressed(KeyCode::S) {
        save_writer.send(SaveEvent {
            filename: SAVE_FILENAME.to_string(),
            location: StorageLocation::Assets,
        });
    }

    // Load the level
    if is_control && !is_shift && keys.just_pressed(KeyCode::L) {
        load_writer.send(LoadEvent {
            filename: SAVE_FILENAME.to_string(),
            location: StorageLocation::Assets,
        });
    }
}

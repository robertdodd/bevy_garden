use bevy::prelude::*;

use bevy_helpers::generic_systems::despawn_recursive_with;
use editor::prelude::*;
use game_state::prelude::*;
use save::prelude::{RollbackBackEvent, RollbackForwardEvent};

use crate::ui::*;

/// Plugin that handles the tool panel while in the game.
pub struct ToolPanelPlugin;

impl Plugin for ToolPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(PlayState::Active), setup_tool_panel)
            .add_systems(
                OnExit(GameState::Game),
                despawn_recursive_with::<OnToolPanel>,
            )
            .add_systems(Update, handle_button_interaction_actions);
    }
}

/// Marker component for despawning the tool panel
#[derive(Component)]
struct OnToolPanel;

/// A tool button in the tool panel
#[derive(Component)]
enum ToolButtonAction {
    Tool(Tool),
    Undo,
    Redo,
}

/// System that spawns and handles the tool panel when in game
fn setup_tool_panel(
    mut commands: Commands,
    tool_library: Res<ToolLibrary>,
    button_style: Res<ToolButtonStyle>,
) {
    spawn_tool_panel(
        &mut commands,
        (GameMarker, OnToolPanel, PickableBlock),
        |p| {
            // Heading
            spawn_tool_panel_heading("Tools", (), p);

            // Tool Buttons from the tool library
            for tool in tool_library.tools.iter() {
                spawn_tool_button(
                    &tool.name,
                    ToolButtonAction::Tool(tool.tool.clone()),
                    &button_style,
                    false,
                    p,
                );
            }

            // Extra Buttons
            spawn_tool_panel_heading("Undo/Redo", (), p);
            spawn_tool_button(
                "Undo (CTRL + Z)",
                ToolButtonAction::Undo,
                &button_style,
                false,
                p,
            );
            spawn_tool_button(
                "Redo (CTRL + Y)",
                ToolButtonAction::Redo,
                &button_style,
                true,
                p,
            );
        },
    );
}

/// System that handles tool button click actions
#[allow(clippy::type_complexity)]
fn handle_button_interaction_actions(
    mut query: Query<(&ToolButtonAction, &Interaction), Changed<Interaction>>,
    mut tool_stack: ResMut<ToolStack>,
    mut undo_writer: EventWriter<RollbackBackEvent>,
    mut redo_writer: EventWriter<RollbackForwardEvent>,
) {
    for (action, interaction) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                ToolButtonAction::Tool(tool) => {
                    tool_stack.clear();
                    tool_stack.push(tool.clone());
                }
                ToolButtonAction::Undo => undo_writer.send(RollbackBackEvent),
                ToolButtonAction::Redo => redo_writer.send(RollbackForwardEvent),
            }
        }
    }
}

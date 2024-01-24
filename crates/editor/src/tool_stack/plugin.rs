use bevy::{input::InputSystem, prelude::*};

use game_state::prelude::*;

use crate::prelude::*;

use super::types::*;

/// Plugin responsible for managing the `ToolStack` resource and handling events for interacting with it.
pub struct ToolStackPlugin;

impl Plugin for ToolStackPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ToolChangedEvent>()
            .add_event::<PushToolEvent>()
            .add_event::<PopToolEvent>()
            .add_event::<ReplaceToolEvent>()
            .insert_resource(ToolStack::new(Tool::None))
            .configure_sets(
                Update,
                (
                    ToolStackSet::EventHandling.after(InputSystem),
                    ToolStackSet::HandleChanges,
                )
                    .chain(),
            )
            .add_systems(OnEnter(GameState::Game), initialize_tool_stack)
            .add_systems(
                Update,
                (
                    (
                        handle_pop_tool_events.run_if(on_event::<PopToolEvent>()),
                        handle_tool_push_events.run_if(on_event::<PushToolEvent>()),
                        handle_replace_tool_events.run_if(on_event::<ReplaceToolEvent>()),
                    )
                        // IMPORTANT: Chain so that Pop events are handled before Push events. This allows a
                        // "Pop-then-push" pattern equivalent to replacing the current tool on the stack.
                        .chain()
                        .in_set(ToolStackSet::EventHandling),
                    handle_tool_stack_changed
                        .run_if(resource_changed::<ToolStack>())
                        .in_set(ToolStackSet::HandleChanges),
                ),
            );
    }
}

/// Initialize the tool state when editor state is entered
fn initialize_tool_stack(mut tool_stack: ResMut<ToolStack>) {
    tool_stack.clear();
    tool_stack.push(Tool::Pointer);
}

/// System that handle events used to push a tool onto the tool stack
fn handle_tool_push_events(
    mut events: EventReader<PushToolEvent>,
    mut tool_stack: ResMut<ToolStack>,
) {
    for event in events.read() {
        info!("[Tool Stack] ==> push tool event: {:?}", event.0.clone());
        tool_stack.push(event.0.clone());
    }
}

/// System that handles events used to replace the current tool on the tool stack
fn handle_replace_tool_events(
    mut events: EventReader<ReplaceToolEvent>,
    mut tool_stack: ResMut<ToolStack>,
) {
    for event in events.read() {
        info!("[Tool Stack] ==> replace tool event: {:?}", event.0.clone());
        tool_stack.pop();
        tool_stack.push(event.0.clone());
    }
}

/// System that handles events used to pop the last tool from the tool stack
fn handle_pop_tool_events(
    mut events: EventReader<PopToolEvent>,
    mut tool_stack: ResMut<ToolStack>,
) {
    for _ in events.read() {
        info!("[Tool Stack] ==> pop tool event");
        tool_stack.pop();
    }
}

/// System that emits `ToolChangedEvent` events whenever the tool stack changes.
fn handle_tool_stack_changed(
    tool_stack: Res<ToolStack>,
    mut tool_changed_writer: EventWriter<ToolChangedEvent>,
) {
    tool_changed_writer.send(ToolChangedEvent(tool_stack.get()));
}

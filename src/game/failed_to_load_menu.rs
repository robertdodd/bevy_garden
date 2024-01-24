use bevy::{app::AppExit, prelude::*};

use bevy_helpers::generic_systems::{debug_state_changes, despawn_recursive_with};
use game_state::prelude::*;

use crate::ui::*;

/// Plugin that spawns a "Failed to Load" menu when the game fails to load.
///
/// In this example, this should only happen if there is an issue with one of the tool assets. We can't get the error
/// messages in the game, so you'll need to refer to the console output to see what went wrong.
pub struct FailedToLoadMenuPlugin;

impl Plugin for FailedToLoadMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(
                OnEnter(PlayState::FailedToLoad),
                transition_to_failed_to_load,
            )
            .add_systems(OnEnter(MenuState::FailedToLoad), setup_menu)
            .add_systems(OnExit(GameState::Menu), on_leave_menu)
            .add_systems(
                OnExit(MenuState::FailedToLoad),
                despawn_recursive_with::<OnFailedToLoadMenu>,
            )
            .add_systems(
                Update,
                (
                    handle_button_interaction_actions.run_if(in_state(MenuState::FailedToLoad)),
                    debug_state_changes::<MenuState>,
                ),
            );
    }
}

/// State for the menu we're on. It should only apply while we are in `GameState::Menu`.
/// NOTE: Usually this would be in "menu" plugin, but this is the only menu in this example so we have it here.
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MenuState {
    #[default]
    None,
    FailedToLoad,
}

/// Marker component for despawning the tool panel
#[derive(Component)]
struct OnFailedToLoadMenu;

/// A tool button in the tool panel
#[derive(Component)]
enum FailedToLoadButtonAction {
    Retry,
    Quit,
}

/// System that transitions to the "failed to load" menu when we enter `PlayState::FailedToLoad`.
fn transition_to_failed_to_load(
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
) {
    next_game_state.set(GameState::Menu);
    next_menu_state.set(MenuState::FailedToLoad);
}

/// System that clears `MenuState` when we leave `GameState::Menu`
fn on_leave_menu(mut next_menu_state: ResMut<NextState<MenuState>>) {
    next_menu_state.set(MenuState::None);
}

/// System that spawns the tool panel when entering the game
fn setup_menu(mut commands: Commands, button_style: Res<ToolButtonStyle>) {
    error!("Setup failed to load menu");

    // Spawn menu camera
    commands.spawn((
        Camera2dBundle::default(),
        OnFailedToLoadMenu,
        Name::new("Menu Camera"),
    ));

    // Spawn menu screen
    spawn_menu_screen(
        &mut commands,
        (GameMarker, OnFailedToLoadMenu, PickableBlock),
        |p| {
            spawn_menu_screen_heading("Failed To Load", (), p);
            spawn_menu_screen_text(
                "There is probably an error in one of the prefab tool assets.",
                (),
                p,
            );
            spawn_menu_screen_text("See logs for more.", (), p);
            spawn_tool_button(
                "Retry",
                FailedToLoadButtonAction::Retry,
                &button_style,
                false,
                p,
            );
            spawn_tool_button(
                "Quit",
                FailedToLoadButtonAction::Quit,
                &button_style,
                true,
                p,
            );
        },
    );
}

/// System that menu button click actions
#[allow(clippy::type_complexity)]
fn handle_button_interaction_actions(
    mut query: Query<(&FailedToLoadButtonAction, &Interaction), Changed<Interaction>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    for (action, interaction) in query.iter_mut() {
        if *interaction == Interaction::Pressed {
            match action {
                FailedToLoadButtonAction::Retry => next_game_state.set(GameState::ReloadLevel),
                FailedToLoadButtonAction::Quit => app_exit_events.send(AppExit),
            }
        }
    }
}

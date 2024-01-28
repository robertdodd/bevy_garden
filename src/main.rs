use bevy::{
    prelude::*,
    render::texture::{ImageAddressMode, ImageSamplerDescriptor},
};
use bevy_mod_raycast::prelude::*;

use apple::prelude::ApplePlugin;
use background::prelude::BackgroundPlugin;
use bush::prelude::BushPlugin;
use duplicate_tool::prelude::DuplicateToolPlugin;
use editor::prelude::EditorPlugin;
use flower::prelude::FlowerPlugin;
use game::GamePlugin;
use game_effects::prelude::GameEffectsPlugin;
use game_state::prelude::{GameState, GameStatePlugin, PlayState};
use pointer_tool::prelude::PointerToolPlugin;
use prefab_tool::prelude::PrefabToolPlugin;
use save::prelude::*;
use tree::prelude::TreePlugin;
use widgets::WidgetsPlugin;

mod config;
mod game;
mod widgets;

fn main() {
    App::new()
        // Bevy + third-party plugins
        .add_plugins((
            DefaultPlugins
                // set all textures to be repeated
                .set(ImagePlugin {
                    default_sampler: ImageSamplerDescriptor {
                        address_mode_u: ImageAddressMode::Repeat,
                        address_mode_v: ImageAddressMode::Repeat,
                        address_mode_w: ImageAddressMode::Repeat,
                        ..Default::default()
                    },
                })
                // configure the window
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: config::GAME_TITLE.to_string(),
                        resolution: (1280., 720.).into(),
                        // Bind to canvas included in `index.html`
                        canvas: Some("#bevy".to_owned()),
                        // Tells wasm to override default event handling, like F5 and Ctrl+R etc.
                        prevent_default_event_handling: true,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
            DefaultRaycastingPlugin,
        ))
        // core workspace plugins
        .add_plugins((
            GameStatePlugin,
            GameEffectsPlugin,
            TreePlugin,
            ApplePlugin,
            SavePlugin,
            BackgroundPlugin,
            EditorPlugin,
            PrefabToolPlugin,
            FlowerPlugin,
            PointerToolPlugin,
            BushPlugin,
            DuplicateToolPlugin,
        ))
        // first-party plugins for in-game menus (tool panel UI and failed-to-load menu), spawning the game and
        // loading the tool assets.
        .add_plugins((GamePlugin, WidgetsPlugin))
        .add_systems(Startup, startup)
        .add_systems(Update, handle_keypress.run_if(in_state(PlayState::Active)))
        .run();
}

/// System that instantly transitions to game state upon startup.
/// NOTE: Usually this would happen when you click "Play" from a menu, but we don't have a menu in this example.
fn startup(mut next_state: ResMut<NextState<GameState>>) {
    // NOTE: Transitioning to `GameState::Game` is all that is required to initiate the loading and setup states.
    //
    // The `GameStatePlugin` will automatically transition between all the `PlayState` states, which includes
    // loading assets, resources, the level and setting up the game entities.
    //
    // The only interaction needed during those state transitions are:
    //
    // * `PlayState::LoadLevel` - this is where you would either load the level asset or create a new level, and then
    //   transition to `PlayState::SetupLevel` when done (or `PlayState::FailedToLoad` if it failed). In this example,
    //   we spawn a new level in [src/game/new_level.rs](src/game/new_level.rs).
    // * `PlayState::FailedToLoad` - this is where you would transition to a "failed-to-load" menu. We do this in
    //   [src/game/failed_to_load_menu.rs](src/game/failed_to_load_menu.rs).
    next_state.set(GameState::Game);
}

/// System that handles key presses while in the game, namely Undo/Redo.
fn handle_keypress(
    keys: Res<Input<KeyCode>>,
    mut roll_back_writer: EventWriter<RollbackBackEvent>,
    mut roll_forward_writer: EventWriter<RollbackForwardEvent>,
    mut rollback_save_writer: EventWriter<RollbackSaveEvent>,
) {
    if keys.pressed(KeyCode::ControlLeft) || keys.pressed(KeyCode::ControlRight) {
        if keys.just_pressed(KeyCode::Z) {
            roll_back_writer.send(RollbackBackEvent);
        }
        if keys.just_pressed(KeyCode::Y) {
            roll_forward_writer.send(RollbackForwardEvent);
        }
        if keys.just_pressed(KeyCode::S) {
            rollback_save_writer.send(RollbackSaveEvent);
        }
    }
}

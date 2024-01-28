use bevy::prelude::*;

use crate::config::{FONT_SIZE_BASE, FONT_SIZE_LG};

use super::spawn_text_heading;

/// Utility that spawns a menu screen node.
pub fn spawn_menu_screen(
    commands: &mut Commands,
    bundle: impl Bundle,
    children: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn((
            bundle,
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    top: Val::Px(0.),
                    left: Val::Px(0.),
                    bottom: Val::Px(0.),
                    right: Val::Px(0.),
                    ..default()
                },
                background_color: Color::GRAY.into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
        ))
        .with_children(|p| {
            p.spawn(NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    ..default()
                },
                ..default()
            })
            .with_children(|p| children(p));
        });
}

/// Utility that spawns a heading styled for the menu screen.
pub fn spawn_menu_screen_heading(
    text: impl Into<String>,
    bundle: impl Bundle,
    child_builder: &mut ChildBuilder,
) -> Entity {
    spawn_text_heading(text, FONT_SIZE_LG, 10., bundle, child_builder)
}

/// Utility that spawns a text node styled for the menu screen.
pub fn spawn_menu_screen_text(
    text: impl Into<String>,
    bundle: impl Bundle,
    child_builder: &mut ChildBuilder,
) -> Entity {
    child_builder
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(10.)),
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            bundle,
        ))
        .with_children(|p| {
            p.spawn(TextBundle::from_section(
                text,
                TextStyle {
                    font_size: FONT_SIZE_BASE,
                    ..default()
                },
            ));
        })
        .id()
}

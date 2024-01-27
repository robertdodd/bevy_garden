use bevy::prelude::*;

use crate::ui::*;

/// System that spawns the tool panel when entering the game
pub fn spawn_tool_panel(
    commands: &mut Commands,
    bundle: impl Bundle,
    children: impl FnOnce(&mut ChildBuilder),
) {
    commands
        .spawn((
            bundle,
            NodeBundle {
                style: Style {
                    flex_direction: FlexDirection::Column,
                    position_type: PositionType::Absolute,
                    top: Val::Px(10.),
                    left: Val::Px(10.),
                    width: Val::Px(200.),
                    border: UiRect::all(Val::Px(1.)),
                    padding: UiRect::all(Val::Px(10.)),
                    ..default()
                },
                background_color: Color::BLACK.into(),
                border_color: Color::WHITE.into(),
                ..default()
            },
        ))
        .with_children(|p| {
            children(p);
        });
}

pub fn spawn_tool_panel_heading(
    text: impl Into<String>,
    bundle: impl Bundle,
    child_builder: &mut ChildBuilder,
) -> Entity {
    spawn_text_heading(text, FONT_SIZE_LG, 10., bundle, child_builder)
}

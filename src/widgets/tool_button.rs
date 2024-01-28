use bevy::prelude::*;

use crate::config::FONT_SIZE_BASE;

/// Plugin that handles tool buttons
pub struct ToolButtonPlugin;

impl Plugin for ToolButtonPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ToolButtonStyle>()
            .add_systems(Update, handle_button_interaction_styles);
    }
}

/// A tool button. Must be added to a UI node.
#[derive(Component)]
struct ToolButton;

/// Resource that holds styling for tool buttons
#[derive(Resource)]
pub struct ToolButtonStyle {
    default_background: Color,
    default_border: Color,
    hover_background: Color,
    hover_border: Color,
    press_background: Color,
    press_border: Color,
    padding: UiRect,
    spacer: Val,
}

impl Default for ToolButtonStyle {
    fn default() -> Self {
        Self {
            default_background: Color::DARK_GRAY,
            default_border: Color::WHITE,
            hover_background: Color::GRAY,
            hover_border: Color::WHITE,
            press_background: Color::DARK_GRAY,
            press_border: Color::DARK_GRAY,
            padding: UiRect::all(Val::Px(10.)),
            spacer: Val::Px(10.),
        }
    }
}

/// System that updates button colors when their interaction changes
#[allow(clippy::type_complexity)]
fn handle_button_interaction_styles(
    mut query: Query<
        (&mut BackgroundColor, &mut BorderColor, &Interaction),
        (With<ToolButton>, Changed<Interaction>),
    >,
    button_style: Res<ToolButtonStyle>,
) {
    for (mut bg, mut border, interaction) in query.iter_mut() {
        let (new_bg, new_border) = match *interaction {
            Interaction::Pressed => (button_style.press_background, button_style.press_border),
            Interaction::Hovered => (button_style.hover_background, button_style.hover_border),
            Interaction::None => (button_style.default_background, button_style.default_border),
        };
        *bg = new_bg.into();
        *border = new_border.into();
    }
}

/// Utility that spawns a tool button
pub fn spawn_tool_button(
    text: impl Into<String>,
    bundle: impl Bundle,
    button_style: &ToolButtonStyle,
    is_last: bool,
    child_builder: &mut ChildBuilder,
) -> Entity {
    child_builder
        .spawn((
            ToolButton,
            bundle,
            ButtonBundle {
                style: Style {
                    padding: button_style.padding,
                    border: UiRect::all(Val::Px(1.)),
                    margin: if is_last {
                        UiRect::default()
                    } else {
                        UiRect::bottom(button_style.spacer)
                    },
                    ..default()
                },
                background_color: button_style.default_background.into(),
                border_color: button_style.default_border.into(),
                ..default()
            },
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

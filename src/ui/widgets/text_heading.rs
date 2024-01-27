use bevy::prelude::*;

/// Utility for spawning a text heaading node
pub fn spawn_text_heading(
    text: impl Into<String>,
    font_size: f32,
    margin_bottom: f32,
    bundle: impl Bundle,
    child_builder: &mut ChildBuilder,
) -> Entity {
    child_builder
        .spawn((
            NodeBundle {
                style: Style {
                    margin: UiRect::bottom(Val::Px(margin_bottom)),
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
                    font_size,
                    ..default()
                },
            ));
        })
        .id()
}

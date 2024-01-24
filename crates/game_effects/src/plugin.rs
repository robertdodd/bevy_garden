use bevy::prelude::*;

use crate::{selected::SelectedPlugin, transparency::TransparentMaterialPlugin};

pub struct GameEffectsPlugin;

impl Plugin for GameEffectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((TransparentMaterialPlugin, SelectedPlugin));
    }
}

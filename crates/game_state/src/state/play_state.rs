use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum PlayState {
    #[default]
    None,
    // Load assets here
    LoadAssets,
    // Setup systems related to assets from the previous state here. E.g. Setup resources referencing the assets,
    // update texture asset formats etc.
    SetupAssets,
    // Setup resources, mainly for resources that implement `FromWorld`
    SetupResources,
    // Load the current level, or create a new level
    LoadLevel,
    // Wait for initial level entities to finish setup here
    SetupLevel,
    // The game is active
    Active,
    // Failure state, when assets fail to load or something else goes wrong during the loading/setup states. Usually,
    // you'd handle this in `main` by showing a "failed to load" menu when this state is entered.
    FailedToLoad,
}

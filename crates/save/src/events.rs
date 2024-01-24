use bevy::prelude::*;

use crate::prelude::StorageLocation;

/// Event used to save a rollback checkpoint
#[derive(Event)]
pub struct RollbackSaveEvent;

/// Event used to load the previous rollback
#[derive(Event)]
pub struct RollbackBackEvent;

/// Event used to load the next rollback
#[derive(Event)]
pub struct RollbackForwardEvent;

/// Event used to load the next rollback
#[derive(Event)]
pub struct RollbackLoadEvent;

/// Event used to save the level to a file
#[derive(Event)]
pub struct SaveEvent {
    pub filename: String,
    pub location: StorageLocation,
}

/// Event used to load a level from a file
#[derive(Event)]
pub struct LoadEvent {
    pub filename: String,
    pub location: StorageLocation,
}

/// Event used to clear rollback history
#[derive(Event)]
pub struct RollbackClearEvent;

/// Event emitted by this crate when a level successfully loads.
#[derive(Event, Debug)]
pub struct LevelLoadSuccess(pub String);

/// Event emitted by this crate when a level fails to load.
#[derive(Event, Debug)]
pub struct LevelLoadFail {
    pub path: String,
    pub error: String,
}

/// Event emitted by this plugin when a save event (level or rollback) completes.
#[derive(Event)]
pub enum SaveResult {
    LevelLoad(Result<(), String>),
    LevelSave(Result<String, String>),
    RollbackSave(Result<(), String>),
    RollbackApply(Result<(), String>),
}

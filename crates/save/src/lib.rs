pub mod app;
pub mod commands;
pub mod events;
pub mod plugin;
pub mod registry;
pub mod rollbacks;
pub mod types;

pub mod prelude {
    pub use crate::{app::AppSaveableExt, events::*, plugin::SavePlugin, registry::*, types::*};
}

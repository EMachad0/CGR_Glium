mod app;
mod app_builder;
mod event;
mod plugin;

pub use app::*;
pub use app_builder::*;
pub use event::*;
pub use plugin::*;

pub mod prelude {
    pub use crate::{
        app::App,
        app_builder::AppBuilder,
        event::*,
        plugin::{Plugin, PluginGroup, PluginGroupBuilder},
        CoreStage, StartupStage,
    };
}

#[macro_use]
extern crate arara_logger;

use bevy_ecs::schedule::StageLabel;

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum CoreStage {
    First,
    EventUpdateStage,
    Startup,
    PreUpdate,
    Update,
    PostUpdate,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, StageLabel)]
pub enum StartupStage {
    PreStartup,
    Startup,
    PostStartup,
}

/// An event that indicates the app should exit. This will fully exit the app process.
#[derive(Debug, Clone)]
pub struct AppExit;

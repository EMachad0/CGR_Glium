use arara_app::{Plugin, AppBuilder, StartupStage};
use bevy_ecs::system::IntoSystem;

use crate::particle_system::{init_particles, update_particles};

pub struct ParticleSystemPlugin;

impl Plugin for ParticleSystemPlugin {
    fn build(&self, app_builder: &mut AppBuilder) {
        app_builder
            .add_startup_system_to_stage(StartupStage::PostStartup, init_particles.system())
            .add_system(update_particles.system());
    }
}
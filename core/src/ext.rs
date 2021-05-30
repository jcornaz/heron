#![allow(missing_docs)]
#![deprecated(
    note = "Physics system can be added to the bevy update stage. Use bevy's add_system instead."
)]

use bevy::app::AppBuilder;
use bevy::ecs::schedule::SystemDescriptor;

pub trait AppBuilderExt {
    fn add_physics_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self;
}

impl AppBuilderExt for AppBuilder {
    fn add_physics_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self {
        self.add_system(system)
    }
}

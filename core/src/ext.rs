#![deprecated(
    note = "Physics system can be added to the bevy update stage. Use bevy's add_system instead."
)]

//! Extensions to bevy API

use bevy::app::AppBuilder;
use bevy::ecs::schedule::SystemDescriptor;

/// Extensions for the app builder
#[deprecated(
    note = "Physics system can be added to the bevy update stage. Use bevy's add_system instead."
)]
pub trait AppBuilderExt {
    /// Add a system to the "physics update" stage so that it runs before each physics step.
    ///
    /// This can be used to add systems that modify transform/velocity or other physics components.
    ///    
    /// Typically (and by default) physics steps run at a fixed rate and are out of sync with the bevy update.
    #[deprecated(
        note = "Physics system can be added to the bevy update stage. Use bevy's add_system instead."
    )]
    fn add_physics_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self;
}

#[allow(deprecated)]
impl AppBuilderExt for AppBuilder {
    fn add_physics_system(&mut self, system: impl Into<SystemDescriptor>) -> &mut Self {
        self.add_system(system)
    }
}

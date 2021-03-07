#![allow(clippy::module_name_repetitions)]

//! Extensions to bevy API

use bevy_app::AppBuilder;
use bevy_ecs::{Schedule, System};

/// Extensions for the app builder
pub trait AppBuilderExt {
    /// Add a system to the "physics update" stage, so that it runs before each physics step.
    ///
    /// This can be used to add systems that modify transform/velocity or other physics components.
    ///
    /// Typically (and by default) the physics step run at a fixed rate and are out of sync of the bevy update.
    fn add_physics_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self;
}

impl AppBuilderExt for AppBuilder {
    fn add_physics_system<S: System<In = (), Out = ()>>(&mut self, system: S) -> &mut Self {
        self.stage(crate::stage::ROOT, |schedule: &mut Schedule| {
            schedule.add_system_to_stage(crate::stage::UPDATE, system)
        })
    }
}

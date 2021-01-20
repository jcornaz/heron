#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy_app::{AppBuilder, Plugin};

pub use heron_core::*;
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

pub mod rapier_plugin {
    pub use heron_rapier::*;
}

pub struct PhysicsPlugin {
    rapier: rapier::RapierPlugin,

    #[cfg(feature = "debug")]
    debug: heron_debug::DebugPlugin,
}

impl PhysicsPlugin {
    /// Configure how many times per second the physics world needs to be updated
    pub fn from_steps_per_second(steps_per_second: u8) -> Self {
        Self::from(RapierPlugin::from_steps_per_second(steps_per_second))
    }

    /// Returns a version using the given color to render collision shapes
    #[cfg(feature = "debug")]
    pub fn with_debug_color(mut self, color: bevy_render::color::Color) -> Self {
        self.debug = color.into();
        self
    }
}

impl From<RapierPlugin> for PhysicsPlugin {
    fn from(rapier: RapierPlugin) -> Self {
        Self {
            rapier,

            #[cfg(feature = "debug")]
            debug: Default::default(),
        }
    }
}

impl Default for PhysicsPlugin {
    fn default() -> Self {
        Self::from(RapierPlugin::default())
    }
}

impl From<rapier::rapier::dynamics::IntegrationParameters> for PhysicsPlugin {
    fn from(parameters: IntegrationParameters) -> Self {
        Self::from(RapierPlugin::from(parameters))
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(self.rapier.clone());

        #[cfg(feature = "debug")]
        app.add_plugin(self.debug);
    }
}

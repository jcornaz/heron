#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

//! An ergonomic API to physics in [bevy] 2d and 3d games. (powered by [rapier])
//!
//! [bevy]: https://bevyengine.org
//! [rapier]: https://rapier.rs

use bevy_app::{AppBuilder, Plugin};

pub use heron_core::*;
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

/// Physics behavior powered by [rapier](https://rapier.rs)
///
/// Allow to access the underlying physics world directly
pub mod rapier_plugin {
    pub use heron_rapier::*;
}

/// Plugin to install in order to enable collision detection and physics behavior.
///
/// When creating the plugin, you may choose the number of physics steps per second.
/// For more advanced configuration, you can create the plugin from a rapier `IntegrationParameters` definition.
#[must_use]
pub struct PhysicsPlugin {
    rapier: RapierPlugin,

    #[cfg(feature = "debug")]
    debug: heron_debug::DebugPlugin,
}

impl PhysicsPlugin {
    /// Configure how many times per second the physics world needs to be updated
    ///
    /// # Panic
    ///
    /// Panic if the number of `steps_per_second` is 0
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

impl From<rapier_plugin::rapier::dynamics::IntegrationParameters> for PhysicsPlugin {
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

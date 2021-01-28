#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value, clippy::needless_doctest_main)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

//! An ergonomic API to physics in [bevy] 2d and 3d games. (powered by [rapier])
//!
//! [bevy]: https://bevyengine.org
//! [rapier]: https://rapier.rs
//!
//! # Get started
//!
//! ## Add the dependency
//!
//! Add the library to `Cargo.toml`
//! ```toml
//! heron = "0.1.0-dev"
//! ```
//!
//! If you are creating a 2d game, change the default features:
//! ```toml
//! heron = { version = "0.1.0-dev", default-features = false, features = ["2d"] }
//! ```
//!
//! Note: when debugging you may consider enabling the `debug` feature, to render the collision shapes (works only for 2d, at the moment).
//!
//! ## Install the plugin
//!
//! To enable physics and collision detection, the [`PhysicsPlugin`] should be installed
//!
//! ```no_run
//! use bevy::prelude::*;
//! use heron::prelude::*;
//!
//! fn main() {
//!   App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(PhysicsPlugin::default())
//!     // ... Add your resources and systems
//!     .run();
//! }
//! ```
//!
//! ## Create rigid bodies
//!
//! To create a rigid body, add the component `Body` to the entity, choosing a collision shape.
//! It will turn the entity into a dynamic rigid body affected by physics.
//!
//! The position, and rotation is defined by the bevy `GlobalTransform` component.
//!
//! ```
//! # use bevy::prelude::*;
//! # use heron::prelude::*;
//! fn spawn(commands: &mut Commands) {
//!     commands
//!         .spawn(SpriteBundle::default()) // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
//!         .with(Body::Sphere { radius: 10.0 }) // Make it a physics body, by attaching a collision shape
//!         .with(Velocity::from(Vec2::unit_x() * 2.0)); // Optionally add a velocity component
//! }
//! ```
//!
//! It is also possible to:
//! * Define the world's [`Gravity`]
//! * listen to [`CollisionEvent`]
//! * Define body's [`Restitution`]

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

/// Re-exports of the most commons/useful types
pub mod prelude {
    pub use crate::{
        AxisAngle, Body, CollisionEvent, Gravity, PhysicsPlugin, Restitution, Velocity,
    };
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

#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value, clippy::needless_doctest_main)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]
//! An ergonomic physics API for 2d and 3d [bevy] games. (powered by [rapier])
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
//! heron = "0.3.0"
//! ```
//!
//! If you are creating a 2d game, change the default features:
//! ```toml
//! heron = { version = "0.3.0", default-features = false, features = ["2d"] }
//! ```
//!
//! Note: when debugging, you may consider enabling the `debug` feature to render the collision shapes (works only for 2d, at the moment).
//!
//! ## Install the plugin
//!
//! The [`PhysicsPlugin`] should be installed to enable physics and collision detection.
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
//! The position and rotation are defined by the bevy `GlobalTransform` component.
//!
//! ```
//! # use bevy::prelude::*;
//! # use heron::prelude::*;
//! fn spawn(mut commands: Commands) {
//!     commands
//!
//!         // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
//!         .spawn(SpriteBundle::default())
//!
//!         // Make it a physics body, by attaching a collision shape
//!         .with(Body::Sphere { radius: 10.0 })
//!
//!         // Optionally define a type (if absent, the body will be *dynamic*)
//!         .with(BodyType::Kinematic)
//!         
//!         // Optionally define the velocity (works only with dynamic and kinematic bodies)
//!         .with(Velocity::from(Vec2::unit_x() * 2.0));
//! }
//! ```
//!
//! ## Run systems synchronously with the physics step
//!
//! The physics step runs at a fixed rate (60 times per second by default) and is out of sync of the
//! bevy frame.
//!
//! But modifying any physics component (such as the transform or velocity), **must** be done synchronously with
//! the physics step.
//!
//! The simplest way is to add these systems with `add_physics_system`:
//!
//! ```no_run
//! # use bevy::prelude::*;
//! # use heron::prelude::*;
//! App::build()
//!     .add_plugins(DefaultPlugins)
//!     .add_plugin(PhysicsPlugin::default())
//!
//!     // This system should NOT update transforms, velocities and other physics components
//!     // In other game engines this would be the "update" function
//!     .add_system(cannot_update_physics.system())
//!
//!     // This system can update transforms, velocities and other physics components
//!     // In other game engines this would be the "physics update" function
//!     .add_physics_system(update_velocities.system())
//! #    .run();
//! # fn cannot_update_physics() {}
//! # fn update_velocities() {}
//! ```
//!
//! ## Move rigid bodies programmatically
//!
//! When creating games, it is often useful to interact with the physics engine and move bodies programmatically.
//! For this, you have two options: Updating the `Transform` or applying a [`Velocity`].
//!
//! ### Option 1: Update the Transform
//!
//! For kinematic bodies ([`BodyType::Kinematic`]), if the transform is updated,
//! the body is moved and get an automatically calculated velocity. Physics rules will be applied normally.
//! Updating the transform is a good way to move a kinematic body.
//!
//! For other types of bodies, if the transform is updated,
//! the rigid body will be *teleported* to the new position/rotation, **ignoring physic rules**.
//!
//! ### Option 2: Use the Velocity component
//!
//! For [`BodyType::Dynamic`] and [`BodyType::Kinematic`] bodies **only**, one can add a [`Velocity`] component to the entity,
//! that will move the body over time. Physics rules will be applied normally.
//!
//! Note that the velocity component is updated by heron to always reflects the current velocity.
//!
//! Defining/updating the velocity is a good way to interact with dynamic bodies.
//!
//! ## See also
//!
//! * How to choose a [collision shape](Body)
//! * How to define a [`BodyType`] (dynamic, static, kinematic or sensor)
//! * How to define the world's [`Gravity`]
//! * How to define the [`PhysicMaterial`]
//! * How to listen to [`CollisionEvent`]
//! * How to define [`RotationConstraints`]

use bevy::app::{AppBuilder, Plugin};

pub use heron_core::*;
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

/// Physics behavior powered by [rapier](https://rapier.rs)
///
/// Allow access to the underlying physics world directly
pub mod rapier_plugin {
    pub use heron_rapier::*;
}

/// Re-exports of the most commons/useful types
pub mod prelude {
    pub use crate::{
        ext::*, stage, Acceleration, AxisAngle, Body, BodyType, CollisionEvent, Gravity,
        PhysicMaterial, PhysicsPlugin, RotationConstraints, Velocity,
    };
}

/// Plugin to install to enable collision detection and physics behavior.
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
    /// # Panics
    ///
    /// Panic if the number of `steps_per_second` is 0
    pub fn from_steps_per_second(steps_per_second: u8) -> Self {
        Self::from(RapierPlugin::from_steps_per_second(steps_per_second))
    }

    /// Returns a version using the given color to render collision shapes
    #[cfg(feature = "debug")]
    pub fn with_debug_color(mut self, color: bevy::render::color::Color) -> Self {
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

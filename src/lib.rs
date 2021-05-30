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
//! heron = "0.6.0"
//! ```
//!
//! If you are creating a 2d game, change the default features:
//! ```toml
//! heron = { version = "0.6.0", default-features = false, features = ["2d"] }
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
//! To create a rigid body, add the `RigdBody` to the entity and add a collision shapes with the `CollisionShape` component.
//!
//! The position and rotation are defined by the bevy `GlobalTransform` component.
//!
//! ```
//! # use bevy::prelude::*;
//! # use heron::prelude::*;
//! fn spawn(mut commands: Commands) {
//! commands
//!
//!     // Spawn any bundle of your choice. Only make sure there is a `GlobalTransform`
//!     .spawn_bundle(SpriteBundle::default())
//!     
//!     // Make it a rigid body
//!     .insert(RigidBody::Dynamic)
//!     
//!     // Attach a collision shape
//!     .insert(CollisionShape::Sphere { radius: 10.0 })
//!     
//!     // Optionally add other useful components...
//!     .insert(Velocity::from_linear(Vec3::X * 2.0))
//!     .insert(Acceleration::from_linear(Vec3::X * 1.0))
//!     .insert(PhysicMaterial { friction: 1.0, density: 10.0, ..Default::default() })
//!     .insert(RotationConstraints::lock());
//! }
//! ```
//!
//! ## Move rigid bodies programmatically
//!
//! When creating games, it is often useful to interact with the physics engine and move bodies programmatically.
//! For this, you have two options: Updating the `Transform` or applying a [`Velocity`].
//!
//! ### Option 1: Update the Transform
//!
//! For kinematic bodies ([`RigidBody::Kinematic`]), if the transform is updated,
//! the body is moved and get an automatically calculated velocity. Physics rules will be applied normally.
//! Updating the transform is a good way to move a kinematic body.
//!
//! For other types of bodies, if the transform is updated,
//! the rigid body will be *teleported* to the new position/rotation, **ignoring physic rules**.
//!
//! ### Option 2: Use the Velocity component
//!
//! For [`RigidBody::Dynamic`] and [`RigidBody::Kinematic`] bodies **only**, one can add a [`Velocity`] component to the entity,
//! that will move the body over time. Physics rules will be applied normally.
//!
//! Note that the velocity component is updated by heron to always reflects the current velocity.
//!
//! Defining/updating the velocity is a good way to interact with dynamic bodies.
//!
//! ## See also
//!
//! * How to define a [`RigidBody`]
//! * How to choose a [`CollisionShape`]
//! * How to define the world's [`Gravity`]
//! * How to define the world's [`PhysicsTime`]
//! * How to define the [`PhysicMaterial`]
//! * How to listen to [`CollisionEvent`]
//! * How to define [`RotationConstraints`]

use bevy::app::{AppBuilder, Plugin};

pub use heron_core::*;
pub use heron_macros::*;
use heron_rapier::RapierPlugin;

/// Physics behavior powered by [rapier](https://rapier.rs)
///
/// Allow access to the underlying physics world directly
pub mod rapier_plugin {
    pub use heron_rapier::*;
}

/// Re-exports of the most commons/useful types
pub mod prelude {
    pub use heron_macros::*;

    #[allow(deprecated)]
    pub use crate::{
        ext::*, stage, Acceleration, AxisAngle, CollisionEvent, CollisionLayers, CollisionShape,
        Gravity, PhysicMaterial, PhysicsLayer, PhysicsPlugin, PhysicsSystem, PhysicsTime,
        RigidBody, RotationConstraints, Velocity,
    };
}

/// Plugin to install to enable collision detection and physics behavior.
#[must_use]
#[derive(Debug, Copy, Clone, Default)]
pub struct PhysicsPlugin {
    #[cfg(feature = "debug")]
    debug: heron_debug::DebugPlugin,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(RapierPlugin);

        #[cfg(feature = "debug")]
        app.add_plugin(self.debug);
    }
}

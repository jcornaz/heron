#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value, clippy::needless_doctest_main)]
#![cfg(any(dim2, dim3))]

//! An ergonomic physics API for 2d and 3d [bevy] games. (powered by [rapier])
//!
//! [bevy]: https://bevyengine.org
//!
//! [rapier]: https://rapier.rs
//!
//! # Get started
//!
//! ## Add the dependency and choose to work with either 2d or 3d
//!
//! Add the library to `Cargo.toml`.
//!
//! For a 3d game:
//! ```toml
//! heron = { version = "0.12.1", features = ["3d"] }
//! ```
//!
//! For as 2d game:
//! ```toml
//! heron = { version = "0.12.1", features = ["2d"] }
//! ```
//!
//! ### Feature flags
//!
//! One must choose to use either `2d` or `3d`. If none of theses two features is enabled, the plugin won't be available.
//!
//! * `3d` Enable simulation on the 3 axes `x`, `y`, and `z`. Incompatible with the feature `2d`.
//! * `2d` Enable simulation only on the first 2 axes `x` and `y`. Incompatible with the feature `3d`, therefore require to disable the default features.
//! * `debug-2d` Render 2d collision shapes. Works only in 2d, support for 3d may be added later.
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
//! To create a rigid body, add the [`RigidBody`] to the entity and add a collision shapes with the
//! [`CollisionShape`] component.
//!
//! The position and rotation are defined by the bevy [`GlobalTransform`] component.
//!
//! [`GlobalTransform`]: bevy::prelude::GlobalTransform
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
//! When creating games, it is often useful to interact with the physics engine and move bodies
//! programmatically. For this, you have two options: Updating the [`Transform`] or applying a
//! [`Velocity`].
//!
//! [`Transform`]: bevy::prelude::Transform
//!
//! ### Option 1: Update the Transform
//!
//! For positional kinematic bodies ([`RigidBody::KinematicPositionBased`]), if the transform is
//! updated, the body is moved and get an automatically calculated velocity. Physics rules will be
//! applied normally. Updating the transform is a good way to move a kinematic body.
//!
//! For other types of bodies, if the transform is updated, the rigid body will be *teleported* to
//! the new position/rotation, **ignoring physic rules**.
//!
//! ### Option 2: Use the Velocity component
//!
//! For [`RigidBody::Dynamic`] and [`RigidBody::KinematicVelocityBased`] bodies **only**, one can
//! add a [`Velocity`] component to the entity, that will move the body over time. Physics rules
//! will be applied normally.
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
//! * How to define [`CustomCollisionShape`] for [`heron_rapier`]

use bevy::app::{App, Plugin};

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
        stage, Acceleration, AxisAngle, CollisionEvent, CollisionLayers, CollisionShape, Gravity,
        PhysicMaterial, PhysicsLayer, PhysicsPlugin, PhysicsSystem, PhysicsTime, RigidBody,
        RotationConstraints, Velocity,
    };
}

/// Plugin to install to enable collision detection and physics behavior.
#[must_use]
#[derive(Debug, Copy, Clone, Default)]
pub struct PhysicsPlugin {
    #[cfg(debug)]
    debug: heron_debug::DebugPlugin,
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(RapierPlugin);

        #[cfg(debug)]
        app.add_plugin(self.debug);
    }
}

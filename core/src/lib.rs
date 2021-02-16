#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]

//! Core components and resources to use Heron

use bevy_ecs::Entity;

use bevy_math::Vec3;
pub use gravity::Gravity;
pub use velocity::{AxisAngle, Velocity};

mod gravity;
pub mod utils;
mod velocity;

/// Components that defines a body subject to physics and collision
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 });
/// }
/// ```
#[derive(Debug, Clone)]
pub enum Body {
    /// A sphere (or circle in 2d) shape defined by its radius
    Sphere {
        /// Radius of the sphere
        radius: f32,
    },

    /// A capsule shape
    Capsule {
        /// Distance from the center of the capsule to the center of an hemisphere.
        half_segment: f32,

        /// Radius of the hemispheres
        radius: f32,
    },

    /// A cuboid/rectangular shape
    Cuboid {
        /// The **half** extends on each axis. (x = half width, y = half height, z = half depth)
        ///
        /// In 2d the `z` axis is ignored
        half_extends: Vec3,
    },
}

/// Component that defines the *type* of rigid body.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(commands: &mut Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 }) // Make a body (is dynamic by default)
///         .with(BodyType::Static); // Make it static (so that it doesn't move and is not affected by forces like gravity)
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum BodyType {
    /// A dynamic body is normally affected by physic forces and affect the other bodies too.
    ///
    /// This is the most "natural" type in the sense that, in the real life, everything is dynamic.
    Dynamic,

    /// A static body is not affected by physic forces and doesn't move. But it does affect the other bodies.
    ///
    /// This effectively behaves like a dynamic body with infinite mass and zero velocity.
    ///
    /// It is especially useful to model terrain and static obstacles.
    Static,

    /// A sensor is not affected by physics forces and doesn't affect other bodies either.
    /// Other bodies will be able to penetrate the sensor.
    ///
    /// A sensor is useful when we are only interested in collision events.
    /// One may for example add a sensor to detect when the player reach a certain area.
    Sensor,
}

impl Default for BodyType {
    fn default() -> Self {
        Self::Dynamic
    }
}

/// An event fired when the collision state between two entities changed
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn detect_collisions(mut events: EventReader<CollisionEvent>) {
///     for event in events.iter() {
///         match event {
///             CollisionEvent::Started(entity1, entity2) => println!("Entity {:?} and {:?} started to collide", entity1, entity2),
///             CollisionEvent::Stopped(entity1, entity2) => println!("Entity {:?} and {:?} stopped to collide", entity1, entity2),
///         }   
///     }   
/// }
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollisionEvent {
    /// The two entities started to collide
    Started(Entity, Entity),

    /// The two entities no longer collide
    Stopped(Entity, Entity),
}

/// Component that define the [Coefficient of Restitution]
///
/// [Coefficient of Restitution]: https://en.wikipedia.org/wiki/Coefficient_of_restitution
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Restitution(f32);

impl Restitution {
    /// Perfectly inelastic coefficient, all kinematic energy is lost on collision. (Do not bounce at all)
    pub const PERFECTLY_INELASTIC: Restitution = Restitution(0.0);

    /// Perfectly elastic coefficient, all kinematic energy is restated in movement. (Very bouncy)
    pub const PERFECTLY_ELASTIC: Restitution = Restitution(1.0);

    /// Create a new restitution from a coefficient value.
    #[must_use]
    pub fn new(coefficient: f32) -> Self {
        Self(coefficient)
    }
}

impl Default for Restitution {
    fn default() -> Self {
        Self::PERFECTLY_INELASTIC
    }
}

impl From<f32> for Restitution {
    fn from(value: f32) -> Self {
        Self(value)
    }
}

impl From<Restitution> for f32 {
    fn from(Restitution(value): Restitution) -> Self {
        value
    }
}

#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]

//! This crate contains the core components and resources to use Heron.

use bevy_ecs::Entity;

pub use gravity::Gravity;
pub use velocity::{AxisAngle, Velocity};

mod gravity;
pub mod utils;
mod velocity;

/// Components that define a body subject to physics and collision
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
}

/// An event fired when the collision state between two entities changed
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn detect_collisions(mut reader: Local<EventReader<CollisionEvent>>, events: Res<Events<CollisionEvent>>) {
///     for event in reader.iter(&events) {
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
    ///
    /// The coefficient must be >= 0
    ///
    /// # Panic
    ///
    /// Panic if the value is bellow 0
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

#![deny(future_incompatible, nonstandard_style)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    clippy::pedantic,
    missing_doc_code_examples
)]

//! This crate contains the core components and resources to use Heron.

pub use gravity::Gravity;
pub use velocity::Angular as AngularVelocity;
pub use velocity::Linear as LinearVelocity;

mod gravity;
mod velocity;

/// Components that define a body subject to physics and collision
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
///
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

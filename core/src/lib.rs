#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]

//! This crate contains the core components and resources to use Heron.

pub use gravity::Gravity;

mod gravity;

/// Components that define a body subject to physics and collision
pub enum Body {
    /// A sphere (or circle in 2d) shape defined by its radius
    Sphere {
        /// Radius of the sphere
        radius: f32,
    },
}

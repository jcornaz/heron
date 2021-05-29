#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::needless_pass_by_value)]

//! Core components and resources to use Heron

use bevy::ecs::schedule::ShouldRun;
use bevy::prelude::*;

pub use constraints::RotationConstraints;
pub use events::{CollisionData, CollisionEvent};
pub use gravity::Gravity;
pub use layers::{CollisionLayers, PhysicsLayer};
pub use physics_time::PhysicsTime;
pub use step::PhysicsSteps;
pub use velocity::{Acceleration, AxisAngle, Velocity};

mod constraints;
mod events;
pub mod ext;
mod gravity;
mod layers;
mod physics_time;
mod step;
pub mod utils;
mod velocity;

/// Physics stages for user systems. These stages are executed once per physics step.
///
/// That usually means they don't run each frame and may run more than once in a single frame.
///
/// In general, end-users shouldn't have to deal with these stages directly.
///
/// Instead, it is possible to call the [`add_physiscs_system`](ext::AppBuilderExt::add_physics_system) extension function on `AppBuilder`
/// to register systems that should run during the physics update.
pub mod stage {

    /// The root **`Schedule`** stage
    pub const ROOT: &str = "heron-physics";

    /// A **child** `SystemStage` running before each physics step.
    ///
    /// Use this stage to modify rigid-body transforms or any other physics component.
    ///
    /// **This is not a root stage**. So you cannot simply call `add_system_to_stage` on bevy's app builder.
    /// Instead consider calling the [`add_physiscs_system`](crate::ext::AppBuilderExt::add_physics_system) extension function.
    pub const UPDATE: &str = "heron-before-step";
}

/// Plugin that registers stage resources and components.
///
/// It does **NOT** enable physics behavior.
#[derive(Debug, Copy, Clone, Default)]
pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Gravity>()
            .init_resource::<PhysicsTime>()
            .init_resource::<PhysicsSteps>()
            .register_type::<CollisionShape>()
            .register_type::<RigidBody>()
            .register_type::<PhysicMaterial>()
            .register_type::<Velocity>()
            .register_type::<Acceleration>()
            .register_type::<RotationConstraints>()
            .register_type::<CollisionLayers>()
            .register_type::<SensorShape>()
            .add_system_to_stage(CoreStage::First, PhysicsSteps::update.system())
            .add_stage_before(CoreStage::PostUpdate, crate::stage::ROOT, {
                Schedule::default()
                    .with_run_criteria(should_run.system())
                    .with_stage(crate::stage::UPDATE, SystemStage::parallel())
            });
    }
}

/// Run criteria system that decides if the physics systems should run.
#[must_use]
pub fn should_run(
    physics_steps: Res<'_, PhysicsSteps>,
    physics_time: Res<'_, PhysicsTime>,
) -> ShouldRun {
    if physics_steps.is_step_frame() && physics_time.scale() > 0.0 {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

/// Components that defines the collision shape of a rigid body
///
/// The collision shape will be attached to the [`RigidBody`] of the same entity.
/// If there isn't any [`RigidBody`] in the entity,
/// the collision shape will be attached to the [`RigidBody`] of the parent entity.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .insert(RigidBody::Dynamic) // Create a dynamic rigid body
///         .insert(CollisionShape::Sphere { radius: 1.0 }); // Attach a collision shape
/// }
#[derive(Debug, Clone, Reflect)]
pub enum CollisionShape {
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

    /// A convex polygon/polyhedron shape
    ConvexHull {
        /// A vector of points describing the convex hull
        points: Vec<Vec3>,
    },
}

impl Default for CollisionShape {
    fn default() -> Self {
        Self::Sphere { radius: 1.0 }
    }
}

/// Component that mark the entity as being a rigid body
///
/// It'll need some [`CollisionShape`] to be attached. Either in the same entity or in a direct child
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .insert(RigidBody::Dynamic) // Create a dynamic rigid body
///         .insert(CollisionShape::Sphere { radius: 1.0 }); // Attach a collision shape
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub enum RigidBody {
    /// A dynamic body is normally affected by physic forces and affect the other bodies normally too.
    ///
    /// This is the most "natural" type in the sense that, in real life, everything is dynamic.
    ///
    /// It is the default type.
    Dynamic,

    /// A static body is not affected by physic forces and doesn't move. But it does affect the other bodies.
    ///
    /// This effectively behaves like a dynamic body with infinite mass and zero velocity.
    ///
    /// It is well suited for terrain and static obstacles.
    Static,

    /// A kinematic body is not moved by the physics engine. But it can have user-defined velocity.
    ///
    /// It affects the other bodies normally but is not affected by them.
    ///
    /// If the transform is updated, then a velocity will be automatically calculated, producing
    /// realistic interaction with other bodies.
    ///
    /// It can also have a velocity be applied.
    ///
    /// It is well-suited for moving platforms.
    Kinematic,

    /// A sensor is not affected by physics forces and doesn't affect other bodies either.
    ///
    /// Other bodies will be able to penetrate the sensor. But it still participates in collision events.
    ///
    /// A sensor is useful when we are only interested in collision events.
    /// One may, for example, add a sensor to detect when the player reaches a certain area.
    Sensor,
}

impl Default for RigidBody {
    fn default() -> Self {
        Self::Dynamic
    }
}

impl RigidBody {
    /// Returns true if this body type can be moved by [`Velocity`]
    #[must_use]
    pub fn can_have_velocity(self) -> bool {
        match self {
            RigidBody::Dynamic | RigidBody::Kinematic => true,
            RigidBody::Static | RigidBody::Sensor => false,
        }
    }
}

/// Mark the [`CollisionShape`] of the same entity as being a *sensor*.
///
/// This is especially useful to add sensor to an existing (non-sensor) rigid body without the need to create a [`RigidBody::Sensor`] in between.
///
/// It has no effect if the concerned rigid body is already a [`RigidBody::Sensor`].
///
/// # Example
///
/// ```rust
/// # use heron_core::*;
/// # use bevy::prelude::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///   commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///     .insert(RigidBody::Dynamic) // <-- A non-sensor rigid body
///     .with_children(|children| {
///       children.spawn_bundle((
///             CollisionShape::Sphere { radius: 1.0 }, // <-- A physics collision shape
///             Transform::default(), // <-- Optionally define it's position
///             GlobalTransform::default(),
///       ));
///  
///       children.spawn_bundle((
///           CollisionShape::Sphere { radius: 1.0 }, // <-- A *sensor* collision shape.
///           SensorShape,
///           Transform::default(), // <-- Optionally define it's position
///           GlobalTransform::default(),
///       ));
///     });
/// }
/// ```
#[derive(Debug, Copy, Clone, Default, Reflect)]
pub struct SensorShape;

/// Component that defines the physics properties of the rigid body
///
/// It must be inserted on the same entity of a [`RigidBody`]
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .insert(CollisionShape::Sphere { radius: 1.0 }) // Make a body (is dynamic by default)
///         .insert(PhysicMaterial {
///             restitution: 0.5, // Define the restitution. Higher value means more "bouncy"
///             density: 2.0, // Define the density. Higher value means heavier.
///             friction: 0.5, // Define the friction. Higher value means higher friction.
///         });
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Reflect)]
pub struct PhysicMaterial {
    /// Coefficient of restitution. Affect how much it "bounces" when colliding with other objects.
    ///
    /// The higher the value, the more "bouncy".
    ///
    /// Typical values are between 0 (perfectly inelastic) and 1 (perfectly elastic)
    pub restitution: f32,

    /// Density. It affects how much the body resists forces.
    ///
    /// The higher the value, the heavier.
    ///
    /// Value must be greater than 0. Except for sensor and static bodies, in which case the value is ignored.
    pub density: f32,

    /// Friction. It affects the relative motion of two bodies in contact.
    ///
    /// The higher the value, the higher friction.
    ///
    /// Typical values are between 0 (ideal) and 1 (max friction)
    pub friction: f32,
}

impl PhysicMaterial {
    /// Perfectly inelastic restitution coefficient, all kinematic energy is lost on collision. (Do not bounce at all)
    pub const PERFECTLY_INELASTIC_RESTITUTION: f32 = 0.0;

    /// Perfectly elastic restitution coefficient, all kinematic energy is restated in movement. (Very bouncy)
    pub const PERFECTLY_ELASTIC_RESTITUTION: f32 = 1.0;
}

impl Default for PhysicMaterial {
    fn default() -> Self {
        Self {
            restitution: Self::PERFECTLY_INELASTIC_RESTITUTION,
            density: 1.0,
            friction: 0.0,
        }
    }
}

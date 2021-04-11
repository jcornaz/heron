#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

//! Core components and resources to use Heron

use bevy::core::FixedTimestep;
use bevy::prelude::*;

pub use constraints::RotationConstraints;
pub use ext::*;
pub use gravity::Gravity;
pub use velocity::{Acceleration, AxisAngle, Velocity};

mod constraints;
pub mod ext;
mod gravity;
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

    /// The root **[`Schedule`](bevy::::ecs::Schedule)** stage
    pub const ROOT: &str = "heron-physics";

    /// A **child** [`SystemStage`](bevy::::ecs::SystemStage) running before each physics step.
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
#[derive(Debug, Copy, Clone)]
pub struct CorePlugin {
    /// Number of physics step per second. `None` means to run physics step as part of the application update instead.
    pub steps_per_second: Option<f64>,
}

impl Default for CorePlugin {
    fn default() -> Self {
        Self::from_steps_per_second(60)
    }
}

impl CorePlugin {
    /// Configure how many times per second the physics world needs to be updated
    ///
    /// # Panics
    ///
    /// Panic if the number of `steps_per_second` is 0
    #[must_use]
    pub fn from_steps_per_second(steps_per_second: u8) -> Self {
        assert!(
            steps_per_second > 0,
            "Invalid number of step per second: {}",
            steps_per_second
        );
        Self {
            steps_per_second: Some(steps_per_second.into()),
        }
    }
}

impl Plugin for CorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<Gravity>()
            .register_type::<Body>()
            .register_type::<BodyType>()
            .register_type::<PhysicMaterial>()
            .register_type::<Velocity>()
            .register_type::<RotationConstraints>()
            .add_stage_before(CoreStage::Update, crate::stage::ROOT, {
                let mut schedule = Schedule::default();

                if let Some(steps_per_second) = self.steps_per_second {
                    schedule = schedule
                        .with_run_criteria(FixedTimestep::steps_per_second(steps_per_second))
                }

                schedule.with_stage(crate::stage::UPDATE, SystemStage::parallel())
            });
    }
}

/// Components that defines a body subject to physics and collision
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 });
/// }
/// ```
#[derive(Debug, Clone, Reflect)]
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

    /// A convex polygon/polyhedron shape
    ConvexHull {
        /// A vector of points describing the convex hull
        points: Vec<Vec3>,
    },
}

impl Default for Body {
    fn default() -> Self {
        Self::Sphere { radius: 1.0 }
    }
}

/// Component that defines the *type* of rigid body.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 }) // Make a body (is dynamic by default)
///         .with(BodyType::Static); // Make it static (so that it doesn't move and is not affected by forces like gravity)
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Reflect)]
pub enum BodyType {
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

impl Default for BodyType {
    fn default() -> Self {
        Self::Dynamic
    }
}

impl BodyType {
    /// Returns true if this body type can be moved by [`Velocity`]
    #[must_use]
    pub fn can_have_velocity(self) -> bool {
        match self {
            BodyType::Dynamic | BodyType::Kinematic => true,
            BodyType::Static | BodyType::Sensor => false,
        }
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
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollisionEvent {
    /// The two entities started to collide
    Started(Entity, Entity),

    /// The two entities no longer collide
    Stopped(Entity, Entity),
}

/// Component that defines the physics properties of the rigid body
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
///     commands.spawn_bundle(todo!("Spawn your sprite/mesh, incl. at least a GlobalTransform"))
///         .with(Body::Sphere { radius: 1.0 }) // Make a body (is dynamic by default)
///         .with(PhysicMaterial {
///             restitution: 0.5, // Define the restitution. Higher value means more "bouncy"
///             density: 2.0, // Define the density. Higher value means heavier.
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
        }
    }
}

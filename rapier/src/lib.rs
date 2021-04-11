#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

//! Physics behavior for Heron, using [rapier](https://rapier.rs/)

#[cfg(feature = "2d")]
pub extern crate rapier2d as rapier;
#[cfg(feature = "3d")]
pub extern crate rapier3d as rapier;

use bevy::prelude::*;

use heron_core::CollisionEvent;

use crate::body::HandleMap;
use crate::rapier::dynamics::{
    CCDSolver, IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet,
};
use crate::rapier::geometry::{BroadPhase, ColliderHandle, ColliderSet, NarrowPhase};
pub use crate::rapier::na as nalgebra;
use crate::rapier::pipeline::PhysicsPipeline;

mod acceleration;
mod body;
pub mod convert;
mod pipeline;
mod velocity;

#[allow(unused)]
mod stage {
    pub(crate) const PRE_STEP: &str = "heron-pre-step";
    pub(crate) const STEP: &str = "heron-step";
    pub(crate) const POST_STEP: &str = "heron-post-step";
}

/// Plugin to install in order to enable collision detection and physics behavior, powered by rapier.
///
/// When creating the plugin, you may choose the number of physics steps per second.
/// For more advanced configuration, you can create the plugin from a rapier `IntegrationParameters` definition.
#[must_use]
#[derive(Clone)]
pub struct RapierPlugin {
    /// Number of step per second, None for a step each frame.
    pub step_per_second: Option<f64>,

    /// Integration parameters, incl. delta-time at each step.
    pub parameters: IntegrationParameters,
}

/// Components automatically register, by the plugin that references the body in rapier's world.
///
/// It can be used to get direct access to rapier's world.
#[derive(Debug, Copy, Clone)]
pub struct BodyHandle {
    rigid_body: RigidBodyHandle,
    collider: ColliderHandle,
}

impl RapierPlugin {
    /// Configure how many times per second the physics world needs to be updated.
    ///
    /// # Panics
    ///
    /// Panic if the number of `steps_per_second` is 0.
    pub fn from_steps_per_second(steps_per_second: u8) -> Self {
        assert!(
            steps_per_second > 0,
            "Invalid number of step per second: {}",
            steps_per_second
        );
        let parameters = IntegrationParameters {
            dt: 1.0 / f32::from(steps_per_second),
            ..IntegrationParameters::default()
        };

        Self {
            parameters,
            step_per_second: Some(steps_per_second.into()),
        }
    }
}

impl Default for RapierPlugin {
    fn default() -> Self {
        Self::from(IntegrationParameters::default())
    }
}

impl From<IntegrationParameters> for RapierPlugin {
    fn from(parameters: IntegrationParameters) -> Self {
        Self {
            #[allow(clippy::cast_possible_truncation)]
            step_per_second: Some(1.0 / f64::from(parameters.dt)),
            parameters,
        }
    }
}

impl Plugin for RapierPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(heron_core::CorePlugin {
            steps_per_second: self.step_per_second,
        })
        .init_resource::<PhysicsPipeline>()
        .init_resource::<HandleMap>()
        .add_event::<CollisionEvent>()
        .insert_resource(self.parameters)
        .insert_resource(BroadPhase::new())
        .insert_resource(NarrowPhase::new())
        .insert_resource(RigidBodySet::new())
        .insert_resource(ColliderSet::new())
        .insert_resource(JointSet::new())
        .insert_resource(CCDSolver::new())
        .stage(heron_core::stage::ROOT, |schedule: &mut Schedule| {
            schedule
                .add_stage(
                    "heron-remove-invalid-bodies",
                    SystemStage::single_threaded().with_system(body::remove_bodies.system()),
                )
                .add_stage(
                    "heron-pre-step",
                    SystemStage::single_threaded()
                        .with_system(
                            bevy::transform::transform_propagate_system::transform_propagate_system
                                .system(),
                        )
                        .with_system(body::remove.system())
                        .with_system(body::update_rapier_position.system())
                        .with_system(velocity::update_rapier_velocity.system())
                        .with_system(body::update_rapier_status.system())
                        .with_system(acceleration::update_rapier_force_and_torque.system())
                        .with_system(body::create.system()),
                )
                .add_stage(
                    "heron-step",
                    SystemStage::single_threaded()
                        .with_system(velocity::apply_velocity_to_kinematic_bodies.system())
                        .with_system(pipeline::step.system()),
                )
                .add_stage(
                    "heron-post-step",
                    SystemStage::parallel()
                        .with_system(body::update_bevy_transform.system())
                        .with_system(velocity::update_velocity_component.system()),
                )
        });
    }
}

impl BodyHandle {
    /// Creates the new `BodyHandle`.
    #[must_use]
    pub fn new(rigid_body: RigidBodyHandle, collider: ColliderHandle) -> BodyHandle {
        BodyHandle {
            rigid_body,
            collider,
        }
    }

    /// Returns the rapier's rigid body handle.
    #[must_use]
    pub fn rigid_body(&self) -> RigidBodyHandle {
        self.rigid_body
    }

    /// Returns the rapier's collider handle.
    #[must_use]
    pub fn collider(&self) -> ColliderHandle {
        self.collider
    }
}

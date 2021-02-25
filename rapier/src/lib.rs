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

use bevy_app::{AppBuilder, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::{IntoChainSystem, IntoSystem, Schedule, SystemStage};

use heron_core::{CollisionEvent, Gravity};

use crate::bodies::HandleMap;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderHandle, ColliderSet, NarrowPhase};
pub use crate::rapier::na as nalgebra;
use crate::rapier::pipeline::PhysicsPipeline;

mod bodies;
pub mod convert;
mod pipeline;
mod restitution;
mod velocity;

#[allow(unused)]
mod stage {
    pub(crate) const START: &str = "heron-start";
    pub(crate) const PRE_STEP: &str = "heron-pre-step";
    pub(crate) const STEP: &str = "heron-step";
    pub(crate) const POST_STEP: &str = "heron-post-step";
    pub(crate) const DEBUG: &str = "heron-debug";
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

/// Components automatically register, by the plugin that reference the body in rapier's world
///
/// It can be used to get direct access to rapier's world.
#[derive(Debug, Copy, Clone)]
pub struct BodyHandle {
    rigid_body: RigidBodyHandle,
    collider: ColliderHandle,
}

impl RapierPlugin {
    /// Configure how many times per second the physics world needs to be updated
    ///
    /// # Panic
    ///
    /// Panic if the number of `steps_per_second` is 0
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
        app.resources_mut().get_or_insert_with(Gravity::default);

        app.init_resource::<PhysicsPipeline>()
            .init_resource::<HandleMap>()
            .add_event::<CollisionEvent>()
            .insert_resource(self.parameters.clone())
            .insert_resource(BroadPhase::new())
            .insert_resource(NarrowPhase::new())
            .insert_resource(RigidBodySet::new())
            .insert_resource(ColliderSet::new())
            .insert_resource(JointSet::new())
            .add_stage_after(
                bevy_app::CoreStage::PostUpdate,
                stage::PRE_STEP,
                SystemStage::single_threaded()
                    .with_system(bodies::remove.system())
                    .with_system(bodies::update_shape.system())
                    .with_system(bodies::update_rapier_position.system())
                    .with_system(velocity::update_rapier_velocity.system())
                    .with_system(restitution::update_rapier_restitution.system())
                    .with_system(bodies::create.system()),
            )
            .add_stage_after(stage::PRE_STEP, "heron-step-and-post-step", {
                let mut schedule = Schedule::default();

                if let Some(steps_per_second) = self.step_per_second {
                    schedule =
                        schedule.with_run_criteria(FixedTimestep::steps_per_second(steps_per_second))
                }

                schedule.with_stage(
                    stage::STEP,
                    SystemStage::single_threaded()
                        .with_system(pipeline::step.system()),
                )
                    .with_stage(
                        stage::POST_STEP,
                        SystemStage::parallel()
                            .with_system(
                                bodies::update_bevy_transform.system().chain(
                                    bevy_transform::transform_propagate_system::transform_propagate_system
                                        .system()
                                )
                            )
                            .with_system(velocity::update_velocity_component.system())
                    )
            });
    }
}

impl BodyHandle {
    /// Returns the rapier's rigid body handle
    #[must_use]
    pub fn rigid_body(&self) -> RigidBodyHandle {
        self.rigid_body
    }

    /// Returns the rapier's collider handle
    #[must_use]
    pub fn collider(&self) -> ColliderHandle {
        self.collider
    }
}

#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value, clippy::type_complexity)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

//! Physics behavior for Heron, using [rapier](https://rapier.rs/)

#[cfg(feature = "2d")]
pub extern crate rapier2d as rapier;
#[cfg(feature = "3d")]
pub extern crate rapier3d as rapier;

use bevy::{ecs::schedule::ShouldRun, prelude::*};

use heron_core::utils::NearZero;
use heron_core::{CollisionEvent, PhysicsTime};

use crate::pipeline::PhysicsStepPerSecond;
use crate::rapier::dynamics::{CCDSolver, IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderSet, NarrowPhase};
pub use crate::rapier::na as nalgebra;
use crate::rapier::pipeline::PhysicsPipeline;

mod acceleration;
mod body;
pub mod convert;
mod pipeline;
mod shape;
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, SystemLabel)]
enum PhysicsSystem {
    TransformPropagation,
    CreateRapierEntity,
    Step,
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
        .init_resource::<body::HandleMap>()
        .init_resource::<shape::HandleMap>()
        .add_event::<CollisionEvent>()
        .insert_resource(self.parameters)
        .insert_resource(BroadPhase::new())
        .insert_resource(NarrowPhase::new())
        .insert_resource(RigidBodySet::new())
        .insert_resource(ColliderSet::new())
        .insert_resource(JointSet::new())
        .insert_resource(CCDSolver::new());

        if let Some(steps_per_second) = self.step_per_second {
            #[allow(clippy::cast_possible_truncation)]
            app.insert_resource(PhysicsStepPerSecond(steps_per_second as f32));
        }

        app.stage(heron_core::stage::ROOT, |schedule: &mut Schedule| {
            schedule
                .add_stage("heron-remove", removal_stage())
                .add_stage("heron-update-rigid-bodies", body_update_stage())
                .add_stage("heron-update-colliders", collider_update_stage())
                .add_stage("heron-step", step_stage())
        });
    }
}

fn should_run(physics_time: Res<'_, PhysicsTime>) -> ShouldRun {
    if physics_time.scale().is_near_zero() {
        ShouldRun::No
    } else {
        ShouldRun::Yes
    }
}

fn removal_stage() -> SystemStage {
    SystemStage::single_threaded()
        .with_system(body::remove_invalids_after_components_removed.system())
        .with_system(shape::remove_invalids_after_components_removed.system())
        .with_system(body::remove_invalids_after_component_changed.system())
        .with_system(shape::remove_invalids_after_component_changed.system())
}

fn body_update_stage() -> SystemStage {
    SystemStage::single_threaded()
        .with_system(
            bevy::transform::transform_propagate_system::transform_propagate_system
                .system()
                .label(PhysicsSystem::TransformPropagation),
        )
        .with_system(
            body::update_rapier_position
                .system()
                .after(PhysicsSystem::TransformPropagation)
                .before(PhysicsSystem::CreateRapierEntity),
        )
        .with_system(
            velocity::update_rapier_velocity
                .system()
                .before(PhysicsSystem::CreateRapierEntity),
        )
        .with_system(
            body::update_rapier_status
                .system()
                .before(PhysicsSystem::CreateRapierEntity),
        )
        .with_system(
            acceleration::update_rapier_force_and_torque
                .system()
                .before(PhysicsSystem::CreateRapierEntity),
        )
        .with_system(
            body::create
                .system()
                .label(PhysicsSystem::CreateRapierEntity)
                .after(PhysicsSystem::TransformPropagation),
        )
}

fn collider_update_stage() -> SystemStage {
    SystemStage::single_threaded()
        .with_system(
            shape::update_position
                .system()
                .before(PhysicsSystem::CreateRapierEntity),
        )
        .with_system(
            shape::create
                .system()
                .label(PhysicsSystem::CreateRapierEntity),
        )
}

fn step_stage() -> SystemStage {
    SystemStage::parallel()
        .with_run_criteria(should_run.system())
        .with_system(
            velocity::apply_velocity_to_kinematic_bodies
                .system()
                .before(PhysicsSystem::Step),
        )
        .with_system(
            pipeline::update_integration_parameters
                .system()
                .before(PhysicsSystem::Step),
        )
        .with_system(pipeline::step.system().label(PhysicsSystem::Step))
        .with_system(
            body::update_bevy_transform
                .system()
                .after(PhysicsSystem::Step),
        )
        .with_system(
            velocity::update_velocity_component
                .system()
                .after(PhysicsSystem::Step),
        )
}

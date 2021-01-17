#![warn(missing_docs)]
#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

//! This crate contains the [`PhysicsPlugin`].

#[cfg(feature = "2d")]
pub extern crate rapier2d as rapier;
#[cfg(feature = "3d")]
pub extern crate rapier3d as rapier;

use bevy_app::{AppBuilder, Plugin};
use bevy_core::FixedTimestep;
use bevy_ecs::{IntoSystem, SystemStage};
use bevy_transform::transform_propagate_system::transform_propagate_system;

use heron_core::Gravity;

use crate::bodies::HandleMap;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderHandle, ColliderSet, NarrowPhase};
pub use crate::rapier::na as nalgebra;
use crate::rapier::pipeline::PhysicsPipeline;

mod bodies;
pub mod convert;
mod pipeline;

#[allow(unused)]
mod stage {
    pub(crate) const START: &str = "heron-start";
    pub(crate) const PRE_STEP: &str = "heron-pre-step";
    pub(crate) const STEP: &str = "heron-step";
}

/// Plugin to install in order to enable collision detection and physics behavior.
///
/// When creating the plugin, you may choose the number of physics steps per second.
/// For more advanced configuration, you can create the plugin from a rapier `IntegrationParameters` definition.
#[derive(Clone, Default)]
pub struct PhysicsPlugin {
    parameters: IntegrationParameters,
}

/// Components automatically register, by the plugin that reference the body in rapier's world
///
/// It can be used to get direct access to rapier's world.
#[derive(Debug, Copy, Clone)]
pub struct BodyHandle {
    rigid_body: RigidBodyHandle,
    collider: ColliderHandle,
}

impl PhysicsPlugin {
    /// Configure how many times per second the physics world needs to be updated
    pub fn with_steps_per_second(steps_per_second: u8) -> Self {
        let mut parameters = IntegrationParameters::default();
        parameters.set_dt(1.0 / steps_per_second as f32);
        Self::from(parameters)
    }
}

impl From<IntegrationParameters> for PhysicsPlugin {
    fn from(parameters: IntegrationParameters) -> Self {
        Self { parameters }
    }
}

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.resources_mut().get_or_insert_with(Gravity::default);

        app.init_resource::<PhysicsPipeline>()
            .init_resource::<HandleMap>()
            .add_resource(self.parameters.clone())
            .add_resource(BroadPhase::new())
            .add_resource(NarrowPhase::new())
            .add_resource(RigidBodySet::new())
            .add_resource(ColliderSet::new())
            .add_resource(JointSet::new())
            .add_stage_after(
                bevy_app::stage::UPDATE,
                stage::START,
                SystemStage::serial().with_system(transform_propagate_system.system()),
            )
            .add_stage_after(
                stage::START,
                stage::PRE_STEP,
                SystemStage::parallel()
                    .with_system(bodies::create.system())
                    .with_system(bodies::update_shape.system())
                    .with_system(bodies::update_rapier_position.system())
                    .with_system(bodies::remove.system()),
            )
            .add_stage_after(stage::PRE_STEP, stage::STEP, {
                let mut stage = SystemStage::serial();

                if self.parameters.dt().is_normal() {
                    stage = stage
                        .with_run_criteria(FixedTimestep::step(self.parameters.dt() as f64))
                        .with_system(pipeline::step.system());
                }

                stage.with_system(bodies::update_bevy_transform.system())
            });
    }
}

impl BodyHandle {
    /// Returns the rapier's rigid body handle
    pub fn rigid_body(&self) -> RigidBodyHandle {
        self.rigid_body
    }
    /// Returns the rapier's collider handle
    pub fn collider(&self) -> ColliderHandle {
        self.collider
    }
}

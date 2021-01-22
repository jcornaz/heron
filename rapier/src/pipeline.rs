use bevy_ecs::{Res, ResMut};

use heron_core::Gravity;

use crate::convert;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use crate::rapier::pipeline::PhysicsPipeline;

#[allow(clippy::too_many_arguments)]
pub(crate) fn step(
    mut pipeline: ResMut<'_, PhysicsPipeline>,
    gravity: Res<'_, Gravity>,
    integration_parameters: Res<'_, IntegrationParameters>,
    mut broad_phase: ResMut<'_, BroadPhase>,
    mut narrow_phase: ResMut<'_, NarrowPhase>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
) {
    let gravity = convert::to_vector(gravity.vector());
    pipeline.step(
        &gravity,
        &integration_parameters,
        &mut broad_phase,
        &mut narrow_phase,
        &mut bodies,
        &mut colliders,
        &mut joints,
        None,
        None,
        &(),
    );
}

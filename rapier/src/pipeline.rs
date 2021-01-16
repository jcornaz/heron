use bevy_ecs::{Res, ResMut};

use heron_core::Gravity;

use crate::convert;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderSet, NarrowPhase};
use crate::rapier::pipeline::PhysicsPipeline;

#[allow(clippy::too_many_arguments)]
pub(crate) fn step(
    mut pipeline: ResMut<PhysicsPipeline>,
    gravity: Res<Gravity>,
    integration_parameters: Res<IntegrationParameters>,
    mut broad_phase: ResMut<BroadPhase>,
    mut narrow_phase: ResMut<NarrowPhase>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
) {
    let gravity = convert::to_vector(gravity.vector());
    let events = ();
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
        &events,
    );
}

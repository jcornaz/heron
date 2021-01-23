use bevy_ecs::{Res, ResMut};
use bevy_math::Vec3;

use heron_core::Gravity;

use crate::convert::IntoRapier;
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
    let gravity = Vec3::from(*gravity).into_rapier();
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

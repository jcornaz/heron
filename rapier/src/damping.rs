use bevy::prelude::*;

use heron_core::Damping;

use crate::rapier::dynamics::{RigidBodyDamping, RigidBodySet};
use crate::RigidBodyHandle;

pub(crate) fn update_rapier_damping(
    mut bodies: ResMut<'_, RigidBodySet>,
    dampings: Query<'_, '_, (&RigidBodyHandle, &Damping), Changed<Damping>>,
) {
    for (handle, damping) in dampings.iter() {
        if let Some(body) = bodies.get_mut(**handle) {
            body.set_linear_damping(damping.linear);
            body.set_angular_damping(damping.angular);
        }
    }
}

pub(crate) fn reset_rapier_damping(
    mut bodies: ResMut<'_, RigidBodySet>,
    handles: Query<'_, '_, &RigidBodyHandle>,
    removed: RemovedComponents<'_, Damping>,
) {
    removed
        .iter()
        .filter_map(|entity| handles.get(entity).ok())
        .for_each(|handle| {
            if let Some(body) = bodies.get_mut(**handle) {
                body.set_linear_damping(RigidBodyDamping::default().linear_damping);
                body.set_angular_damping(RigidBodyDamping::default().angular_damping);
            }
        });
}

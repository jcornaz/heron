use bevy_ecs::prelude::*;

use heron_core::ext::NearZero;
use heron_core::Velocity;

use crate::convert::IntoRapier;
use crate::rapier::dynamics::RigidBodySet;
use crate::BodyHandle;

pub(crate) fn update_rapier_velocity(
    mut bodies: ResMut<'_, RigidBodySet>,
    velocities: Query<'_, (&BodyHandle, &Velocity), Changed<Velocity>>,
) {
    for (handle, velocity) in velocities.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            let wake_up = velocity.is_near_zero();
            body.set_linvel(velocity.linear.into_rapier(), wake_up);
            body.set_angvel(velocity.linear.into_rapier(), wake_up);
        }
    }
}

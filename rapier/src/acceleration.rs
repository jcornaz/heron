use bevy::prelude::*;

use heron_core::{utils::NearZero, Acceleration};
use rapier::{
    dynamics::RigidBody,
    math::{Real, Vector},
};

use crate::convert::IntoRapier;
use crate::rapier::dynamics::RigidBodySet;
use crate::BodyHandle;

pub(crate) fn update_rapier_force_and_torque(
    mut bodies: ResMut<'_, RigidBodySet>,
    accelerations: Query<'_, (&BodyHandle, &Acceleration)>,
) {
    for (handle, acceleration) in accelerations.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            update_acceleration(body, acceleration)
        }
    }
}

pub(crate) fn update_acceleration(body: &mut RigidBody, acceleration: &Acceleration) {
    let wake_up = !acceleration.is_near_zero();
    let linear_acceleration: Vector<Real> = acceleration.linear.into_rapier();
    #[cfg(feature = "3d")]
    let angular_acceleration: Vector<Real> = acceleration.angular.into_rapier();
    #[cfg(feature = "2d")]
    let angular_acceleration: Real = acceleration.angular.into_rapier();
    let inertia = {
        #[cfg(feature = "3d")]
        {
            body.mass_properties().reconstruct_inertia_matrix()
        }
        #[cfg(feature = "2d")]
        {
            let val = body.mass_properties().inv_principal_inertia_sqrt;
            if val == 0.0 {
                0.0
            } else {
                (1.0 / val).powi(2)
            }
        }
    };
    body.apply_force(linear_acceleration * body.mass(), wake_up);
    body.apply_torque(inertia * angular_acceleration, wake_up)
}

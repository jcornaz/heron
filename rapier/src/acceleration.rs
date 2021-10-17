use bevy::prelude::*;

use heron_core::{utils::NearZero, Acceleration};

use crate::convert::IntoRapier;
use crate::rapier::dynamics::{RigidBodyHandle, RigidBodySet};
use crate::rapier::{
    dynamics::RigidBody,
    math::{AngVector, Vector},
};

pub(crate) fn update_rapier_force_and_torque(
    mut bodies: ResMut<'_, RigidBodySet>,
    accelerations: Query<'_, '_, (&super::RigidBodyHandle, &Acceleration)>,
) {
    for (handle, acceleration) in accelerations.iter() {
        if let Some(body) = bodies.get_mut(handle.0) {
            update_acceleration(body, acceleration);
        }
    }
}

fn update_acceleration(body: &mut RigidBody, acceleration: &Acceleration) {
    let wake_up = !acceleration.is_near_zero();
    let linear_acceleration: Vector<f32> = acceleration.linear.into_rapier();
    let angular_acceleration: AngVector<f32> = acceleration.angular.into_rapier();
    let inertia = {
        #[cfg(dim3)]
        {
            body.mass_properties().reconstruct_inertia_matrix()
        }
        #[cfg(dim2)]
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
    body.apply_torque(inertia * angular_acceleration, wake_up);
}

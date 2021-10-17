use bevy::ecs::prelude::*;

use heron_core::utils::NearZero;
use heron_core::{RigidBody, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{RigidBodyHandle, RigidBodySet};

pub(crate) fn update_rapier_velocity(
    mut bodies: ResMut<'_, RigidBodySet>,
    query: Query<
        '_,
        '_,
        (&super::RigidBodyHandle, Option<&RigidBody>, &Velocity),
    >,
) {
    let dynamic_bodies = query
        .iter()
        .filter(|(_, body_type, _)| body_type.copied().unwrap_or_default().can_have_velocity());

    for (handle, _, velocity) in dynamic_bodies {
        if let Some(body) = bodies.get_mut(handle.0) {
            let wake_up = !velocity.is_near_zero();
            body.set_linvel(velocity.linear.into_rapier(), wake_up);
            body.set_angvel(velocity.angular.into_rapier(), wake_up);
        }
    }
}

pub(crate) fn update_velocity_component(
    bodies: Res<'_, RigidBodySet>,
    mut velocities: Query<'_, '_, (&super::RigidBodyHandle, &mut Velocity)>,
) {
    for (handle, mut velocity) in velocities.iter_mut() {
        if let Some(body) = bodies.get(handle.0).filter(|it| it.is_dynamic()) {
            velocity.linear = (*body.linvel()).into_bevy();

            #[cfg(dim2)]
            {
                velocity.angular = heron_core::AxisAngle::from(bevy::math::Vec3::Z * body.angvel());
            }

            #[cfg(dim3)]
            {
                velocity.angular = (*body.angvel()).into_bevy().into();
            }
        }
    }
}

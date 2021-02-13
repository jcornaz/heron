use bevy_ecs::prelude::*;

use heron_core::utils::NearZero;
use heron_core::Velocity;

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::RigidBodySet;
use crate::BodyHandle;

pub(crate) fn update_rapier_velocity(
    mut bodies: ResMut<'_, RigidBodySet>,
    velocities: Query<'_, (&BodyHandle, &Velocity), Changed<Velocity>>,
) {
    for (handle, velocity) in velocities.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            let wake_up = !velocity.is_near_zero();
            body.set_linvel(velocity.linear.into_rapier(), wake_up);
            body.set_angvel(velocity.angular.into_rapier(), wake_up);
        }
    }
}

pub(crate) fn update_velocity_component(
    bodies: Res<'_, RigidBodySet>,
    mut velocities: Query<'_, (&BodyHandle, &mut Velocity)>,
) {
    for (handle, mut velocity) in velocities.iter_mut() {
        if let Some(body) = bodies.get(handle.rigid_body).filter(|it| it.is_dynamic()) {
            velocity.linear = (*body.linvel()).into_bevy();

            #[cfg(feature = "2d")]
            {
                velocity.angular =
                    heron_core::AxisAngle::from(bevy_math::Vec3::unit_z() * body.angvel());
            }

            #[cfg(feature = "3d")]
            {
                velocity.angular = (*body.angvel()).into_bevy().into();
            }
        }
    }
}

use bevy::ecs::prelude::*;
use bevy::math::prelude::*;

use heron_core::utils::NearZero;
use heron_core::{BodyType, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{IntegrationParameters, RigidBodySet};
use crate::BodyHandle;

pub(crate) fn update_rapier_velocity(
    mut bodies: ResMut<'_, RigidBodySet>,
    query: Query<'_, (&BodyHandle, Option<&BodyType>, &Velocity), Changed<Velocity>>,
) {
    let dynamic_bodies = query.iter().filter(|(_, body_type, _)| {
        matches!(body_type.cloned().unwrap_or_default(), BodyType::Dynamic)
    });

    for (handle, _, velocity) in dynamic_bodies {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            let wake_up = !velocity.is_near_zero();
            body.set_linvel(velocity.linear.into_rapier(), wake_up);
            body.set_angvel(velocity.angular.into_rapier(), wake_up);
        }
    }
}

pub(crate) fn apply_velocity_to_kinematic_bodies(
    mut bodies: ResMut<'_, RigidBodySet>,
    integration_parameters: Res<'_, IntegrationParameters>,
    query: Query<'_, (&BodyHandle, &BodyType, &Velocity)>,
) {
    let delta_time = integration_parameters.dt;
    let kinematic_bodies = query
        .iter()
        .filter(|(_, body_type, _)| matches!(body_type, BodyType::Kinematic));

    for (handle, _, velocity) in kinematic_bodies {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            let (mut translation, mut rotation) = body.position().into_bevy();
            translation += velocity.linear * delta_time;
            rotation *= Quat::from(velocity.angular * delta_time);
            body.set_next_kinematic_position((translation, rotation).into_rapier())
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
                velocity.angular = heron_core::AxisAngle::from(bevy::math::Vec3::Z * body.angvel());
            }

            #[cfg(feature = "3d")]
            {
                velocity.angular = (*body.angvel()).into_bevy().into();
            }
        }
    }
}

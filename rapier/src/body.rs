use bevy::ecs::prelude::*;
use bevy::transform::prelude::*;
use fnv::FnvHashMap;

use heron_core::{PhysicMaterial, RigidBody, RotationConstraints, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{
    IslandManager, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet, RigidBodyType,
};
use crate::rapier::geometry::{ColliderHandle, ColliderSet};

pub(crate) type HandleMap = FnvHashMap<Entity, RigidBodyHandle>;

#[allow(clippy::type_complexity)]
pub(crate) fn create(
    mut commands: Commands<'_, '_>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut handles: ResMut<'_, HandleMap>,
    query: Query<
        '_,
        '_,
        (
            Entity,
            &GlobalTransform,
            &RigidBody,
            Option<&Velocity>,
            Option<&RotationConstraints>,
        ),
        Without<super::RigidBodyHandle>,
    >,
) {
    for (entity, transform, body, velocity, rotation_constraints) in query.iter() {
        let mut builder = RigidBodyBuilder::new(body_status(*body))
            .user_data(entity.to_bits().into())
            .position((transform.translation, transform.rotation).into_rapier());

        #[allow(unused_variables)]
        if let Some(RotationConstraints {
            allow_x,
            allow_y,
            allow_z,
        }) = rotation_constraints.copied()
        {
            #[cfg(dim2)]
            if !allow_z {
                builder = builder.lock_rotations();
            }
            #[cfg(dim3)]
            {
                builder = builder.restrict_rotations(allow_x, allow_y, allow_z);
            }
        }

        if let Some(v) = velocity {
            builder = builder
                .linvel(v.linear.into_rapier())
                .angvel(v.angular.into_rapier());
        }

        let rigid_body_handle = bodies.insert(builder.build());

        handles.insert(entity, rigid_body_handle);
        commands
            .entity(entity)
            .insert(super::RigidBodyHandle(rigid_body_handle));
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn remove_invalids_after_components_removed(
    mut commands: Commands<'_, '_>,
    mut handles: ResMut<'_, HandleMap>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut islands: ResMut<'_, IslandManager>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
    bodies_removed: RemovedComponents<'_, RigidBody>,
    constraints_removed: RemovedComponents<'_, RotationConstraints>,
    materials_removed: RemovedComponents<'_, PhysicMaterial>,
) {
    bodies_removed
        .iter()
        .chain(constraints_removed.iter())
        .chain(materials_removed.iter())
        .for_each(|entity| {
            if let Some(handle) = handles.remove(&entity) {
                remove_collider_handles(&mut commands, &bodies, &colliders, handle);
                bodies.remove(handle, &mut islands, &mut colliders, &mut joints);
                commands.entity(entity).remove::<super::RigidBodyHandle>();
            }
        });
}

#[allow(clippy::type_complexity)]
pub(crate) fn remove_invalids_after_component_changed(
    mut commands: Commands<'_, '_>,
    mut handles: ResMut<'_, HandleMap>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut islands: ResMut<'_, IslandManager>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
    changed: Query<
        '_,
        '_,
        (Entity, &super::RigidBodyHandle),
        Or<(
            Changed<RigidBody>,
            Changed<RotationConstraints>,
            Changed<PhysicMaterial>,
        )>,
    >,
) {
    for (entity, handle) in changed.iter() {
        remove_collider_handles(&mut commands, &bodies, &colliders, handle.0);
        bodies.remove(handle.0, &mut islands, &mut colliders, &mut joints);
        commands.entity(entity).remove::<super::RigidBodyHandle>();
        handles.remove(&entity);
    }
}

#[allow(clippy::manual_filter_map)]
fn remove_collider_handles(
    commands: &mut Commands<'_, '_>,
    bodies: &RigidBodySet,
    colliders: &ColliderSet,
    handle: RigidBodyHandle,
) {
    bodies
        .get(handle)
        .iter()
        .flat_map(|it| it.colliders().iter())
        .filter_map(|it| colliders.get(*it))
        .map(|it| {
            #[allow(clippy::cast_possible_truncation)]
            Entity::from_bits(it.user_data as u64)
        })
        .for_each(|collider_entity| {
            commands
                .entity(collider_entity)
                .remove::<super::ColliderHandle>();
        });
}

pub(crate) fn update_rapier_position(
    mut bodies: ResMut<'_, RigidBodySet>,
    query: Query<'_, '_, (&GlobalTransform, &super::RigidBodyHandle), Changed<GlobalTransform>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(body) = bodies.get_mut(handle.0) {
            let isometry = (transform.translation, transform.rotation).into_rapier();
            if body.is_kinematic() {
                body.set_next_kinematic_position(isometry);
            } else {
                body.set_position(isometry, true);
            }
        }
    }
}

pub(crate) fn update_bevy_transform(
    bodies: Res<'_, RigidBodySet>,
    mut query: Query<
        '_,
        '_,
        (
            Option<&mut Transform>,
            &mut GlobalTransform,
            &super::RigidBodyHandle,
            Option<&RigidBody>,
        ),
    >,
) {
    for (mut local, mut global, handle, body_type) in query.iter_mut() {
        if !body_type.copied().unwrap_or_default().can_have_velocity() {
            continue;
        }

        let body = match bodies.get(handle.0) {
            None => continue,
            Some(body) => body,
        };

        #[cfg(dim3)]
        let (translation, rotation) = body.position().into_bevy();
        #[cfg(dim2)]
        let (mut translation, rotation) = body.position().into_bevy();

        #[cfg(dim2)]
        {
            // In 2D, preserve the transform `z` component that may have been set by the user
            translation.z = global.translation.z;
        }

        if translation == global.translation && rotation == global.rotation {
            continue;
        }

        if let Some(local) = &mut local {
            if local.translation == global.translation {
                local.translation = translation;
            } else {
                local.translation = translation - (global.translation - local.translation);
            }

            if local.rotation == global.rotation {
                local.rotation = rotation;
            } else {
                local.rotation =
                    rotation * (global.rotation * local.rotation.conjugate()).conjugate();
            }
        }

        global.translation = translation;
        global.rotation = rotation;
    }
}

fn body_status(body_type: RigidBody) -> RigidBodyType {
    match body_type {
        RigidBody::Dynamic => RigidBodyType::Dynamic,
        RigidBody::Static | RigidBody::Sensor => RigidBodyType::Static,
        RigidBody::KinematicPositionBased => RigidBodyType::KinematicPositionBased,
        RigidBody::KinematicVelocityBased => RigidBodyType::KinematicVelocityBased,
    }
}

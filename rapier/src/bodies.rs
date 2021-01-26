use bevy_ecs::prelude::*;
use bevy_transform::prelude::*;
use fnv::FnvHashMap;

use heron_core::{Body, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{ColliderBuilder, ColliderSet};
use crate::BodyHandle;

pub(crate) type HandleMap = FnvHashMap<Entity, RigidBodyHandle>;

pub(crate) fn create(
    commands: &mut Commands,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut handles: ResMut<'_, HandleMap>,
    query: Query<'_, (Entity, &Body, &GlobalTransform, Option<&Velocity>), Without<BodyHandle>>,
) {
    for (entity, body, transform, velocity) in query.iter() {
        let mut builder = RigidBodyBuilder::new_dynamic()
            .user_data(entity.to_bits().into())
            .position((transform.translation, transform.rotation).into_rapier());

        if let Some(v) = velocity {
            #[cfg(feature = "2d")]
            {
                builder = builder.linvel(v.linear.x, v.linear.y);
            }
            #[cfg(feature = "3d")]
            {
                builder = builder.linvel(v.linear.x, v.linear.y, v.linear.z);
            }

            builder = builder.angvel(v.angular.into_rapier());
        }

        let rigid_body = bodies.insert(builder.build());
        let collider = colliders.insert(
            collider_builder(&body)
                .user_data(entity.to_bits().into())
                .build(),
            rigid_body,
            &mut bodies,
        );
        handles.insert(entity, rigid_body);
        commands.insert_one(
            entity,
            BodyHandle {
                rigid_body,
                collider,
            },
        );
    }
}

pub(crate) fn update_shape(
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut query: Query<'_, (&Body, &mut BodyHandle), Mutated<Body>>,
) {
    for (body_def, mut handle) in query.iter_mut() {
        colliders.remove(handle.collider, &mut bodies, true);
        handle.collider = colliders.insert(
            collider_builder(&body_def).build(),
            handle.rigid_body,
            &mut bodies,
        );
    }
}

pub(crate) fn update_rapier_position(
    mut bodies: ResMut<'_, RigidBodySet>,
    query: Query<'_, (&GlobalTransform, &BodyHandle), Mutated<GlobalTransform>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            body.set_position(
                (transform.translation, transform.rotation).into_rapier(),
                true,
            );
        }
    }
}

pub(crate) fn update_bevy_transform(
    bodies: Res<'_, RigidBodySet>,
    mut query: Query<'_, (Option<&mut Transform>, &mut GlobalTransform, &BodyHandle)>,
) {
    for (mut local, mut global, handle) in query.iter_mut() {
        let body = match bodies.get(handle.rigid_body).filter(|it| it.is_dynamic()) {
            None => continue,
            Some(body) => body,
        };
        let (translation, rotation) = body.position().into_bevy();

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

pub(crate) fn remove(
    commands: &mut Commands,
    mut handles: ResMut<'_, HandleMap>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
    query: Query<'_, (Entity, &BodyHandle), Without<Body>>,
) {
    for entity in query.removed::<Body>() {
        if let Some(handle) = handles.remove(entity) {
            bodies.remove(handle, &mut colliders, &mut joints);
            commands.remove_one::<BodyHandle>(*entity);
        }
    }
}

fn collider_builder(body: &Body) -> ColliderBuilder {
    match body {
        Body::Sphere { radius } => ColliderBuilder::ball(*radius),
    }
}

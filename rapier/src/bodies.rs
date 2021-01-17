use bevy_ecs::*;
use bevy_transform::components::GlobalTransform;

use heron_core::Body;

use crate::rapier::dynamics::{JointSet, RigidBodyBuilder, RigidBodySet};
use crate::rapier::geometry::{ColliderBuilder, ColliderSet};
use crate::{convert, BodyHandle};

pub(crate) fn create(
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    query: Query<(Entity, &Body, &GlobalTransform), Without<BodyHandle>>,
) {
    for (entity, body, transform) in query.iter() {
        let rigid_body = bodies.insert(
            RigidBodyBuilder::new_dynamic()
                .position(convert::to_isometry(
                    transform.translation,
                    transform.rotation,
                ))
                .build(),
        );
        let collider = colliders.insert(collider_builder(&body).build(), rigid_body, &mut bodies);
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
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut query: Query<(&Body, &mut BodyHandle), Mutated<Body>>,
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

pub(crate) fn update_transform(
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&GlobalTransform, &BodyHandle), Mutated<GlobalTransform>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            body.set_position(
                convert::to_isometry(transform.translation, transform.rotation),
                true,
            );
        }
    }
}

pub(crate) fn remove(
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    query: Query<(Entity, &BodyHandle), Without<Body>>,
) {
    for (entity, handle) in query.iter() {
        bodies.remove(handle.rigid_body, &mut colliders, &mut joints);
        commands.remove_one::<BodyHandle>(entity);
    }
}

fn collider_builder(body: &Body) -> ColliderBuilder {
    match body {
        Body::Sphere { radius } => ColliderBuilder::ball(*radius),
    }
}

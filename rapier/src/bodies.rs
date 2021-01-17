use bevy_ecs::*;
use bevy_transform::components::{Children, GlobalTransform, Parent, Transform};
use fnv::FnvHashMap;

use heron_core::Body;

use crate::rapier::dynamics::{JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{ColliderBuilder, ColliderSet};
use crate::{convert, BodyHandle};

pub(crate) type HandleMap = FnvHashMap<Entity, RigidBodyHandle>;

pub(crate) fn create(
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut handles: ResMut<HandleMap>,
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

pub(crate) fn update_rapier_position(
    mut bodies: ResMut<RigidBodySet>,
    query: Query<(&GlobalTransform, &BodyHandle), Mutated<GlobalTransform>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            println!("Update rapier position");
            body.set_position(
                convert::to_isometry(transform.translation, transform.rotation),
                true,
            );
        }
    }
}

pub(crate) fn update_bevy_transform(
    bodies: Res<RigidBodySet>,
    parents: Query<&GlobalTransform, With<Children>>,
    mut query: Query<(
        &mut Transform,
        &GlobalTransform,
        &BodyHandle,
        Option<&Parent>,
    )>,
) {
    for (mut local_transform, global_transform, handle, parent) in query.iter_mut() {
        if let Some(body) = bodies.get(handle.rigid_body).filter(|it| !it.is_sleeping()) {
            let (translation, rotation) = convert::from_isometry(*body.position());

            if translation != global_transform.translation || rotation != global_transform.rotation
            {
                if let Some(parent_transform) =
                    parent.and_then(|Parent(entity)| parents.get(*entity).ok())
                {
                    local_transform.translation = translation - parent_transform.translation;
                    local_transform.rotation = rotation * parent_transform.rotation.conjugate();
                } else {
                    local_transform.translation = translation;
                    local_transform.rotation = rotation;
                }
            }
        }
    }
}

pub(crate) fn remove(
    commands: &mut Commands,
    mut handles: ResMut<HandleMap>,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    mut joints: ResMut<JointSet>,
    query: Query<(Entity, &BodyHandle), Without<Body>>,
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

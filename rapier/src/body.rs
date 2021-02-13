use bevy_ecs::prelude::*;
use bevy_math::prelude::*;
use bevy_transform::prelude::*;
use fnv::FnvHashMap;

use heron_core::{Body, BodyType, Restitution, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{
    BodyStatus, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use crate::rapier::geometry::{Collider, ColliderBuilder, ColliderSet};
use crate::BodyHandle;

pub(crate) type HandleMap = FnvHashMap<Entity, RigidBodyHandle>;

#[allow(clippy::type_complexity)]
pub(crate) fn create(
    commands: &mut Commands,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut handles: ResMut<'_, HandleMap>,
    query: Query<
        '_,
        (
            Entity,
            &Body,
            &GlobalTransform,
            Option<&BodyType>,
            Option<&Velocity>,
            Option<&Restitution>,
        ),
        Without<BodyHandle>,
    >,
) {
    for (entity, body, transform, body_type, velocity, restitution) in query.iter() {
        let body_type = body_type.cloned().unwrap_or_default();

        let mut builder = RigidBodyBuilder::new(body_status(body_type))
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
            build_collider(entity, &body, body_type, restitution.cloned()),
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

#[allow(clippy::type_complexity)]
pub(crate) fn update_shape(
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut query: Query<
        '_,
        (
            Entity,
            &Body,
            &mut BodyHandle,
            Option<&BodyType>,
            Option<&Restitution>,
        ),
        Or<(Mutated<Body>, Changed<BodyType>)>,
    >,
) {
    for (entity, body_def, mut handle, body_type, restitution) in query.iter_mut() {
        colliders.remove(handle.collider, &mut bodies, true);
        handle.collider = colliders.insert(
            build_collider(
                entity,
                &body_def,
                body_type.cloned().unwrap_or_default(),
                restitution.cloned(),
            ),
            handle.rigid_body,
            &mut bodies,
        );
    }
}

pub(crate) fn update_rapier_status(
    mut bodies: ResMut<'_, RigidBodySet>,
    with_type_changed: Query<'_, (&BodyType, &BodyHandle), Changed<BodyType>>,
    without_type: Query<'_, &BodyHandle, Without<BodyType>>,
) {
    for (body_type, handle) in with_type_changed.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            body.body_status = body_status(*body_type);
        }
    }

    for entity in without_type.removed::<BodyType>() {
        if let Some(body) = without_type
            .get(*entity)
            .ok()
            .and_then(|handle| bodies.get_mut(handle.rigid_body))
        {
            body.body_status = body_status(BodyType::default());
        }
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
    mut query: Query<
        '_,
        (
            Option<&mut Transform>,
            &mut GlobalTransform,
            &BodyHandle,
            Option<&BodyType>,
        ),
    >,
) {
    for (mut local, mut global, handle, body_type) in query.iter_mut() {
        if !matches!(body_type.cloned().unwrap_or_default(), BodyType::Dynamic) {
            continue;
        }

        let body = match bodies.get(handle.rigid_body) {
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

fn build_collider(
    entity: Entity,
    body: &Body,
    body_type: BodyType,
    restitution: Option<Restitution>,
) -> Collider {
    let mut builder = match body {
        Body::Sphere { radius } => ColliderBuilder::ball(*radius),
        Body::Capsule {
            half_segment: half_height,
            radius,
        } => ColliderBuilder::capsule_y(*half_height, *radius),
        Body::Cuboid { half_extends } => cuboid_builder(*half_extends),
    };

    builder = builder
        .user_data(entity.to_bits().into())
        .sensor(matches!(body_type, BodyType::Sensor));

    if let Some(restitution) = restitution {
        builder = builder.restitution(restitution.into());
    }

    builder.build()
}

#[inline]
#[cfg(feature = "2d")]
fn cuboid_builder(half_extends: Vec3) -> ColliderBuilder {
    ColliderBuilder::cuboid(half_extends.x, half_extends.y)
}

#[inline]
#[cfg(feature = "3d")]
fn cuboid_builder(half_extends: Vec3) -> ColliderBuilder {
    ColliderBuilder::cuboid(half_extends.x, half_extends.y, half_extends.z)
}

fn body_status(body_type: BodyType) -> BodyStatus {
    match body_type {
        BodyType::Dynamic => BodyStatus::Dynamic,
        BodyType::Static | BodyType::Sensor => BodyStatus::Static,
    }
}

#[cfg(test)]
mod tests {
    use bevy_math::Vec3;

    use super::*;

    #[test]
    fn build_sphere() {
        let builder = build_collider(
            Entity::new(0),
            &Body::Sphere { radius: 4.2 },
            BodyType::default(),
            None,
        );
        let ball = builder
            .shape()
            .as_ball()
            .expect("Created shape was not a ball");
        assert_eq!(ball.radius, 4.2);
    }

    #[test]
    fn build_cuboid() {
        let builder = build_collider(
            Entity::new(0),
            &Body::Cuboid {
                half_extends: Vec3::new(1.0, 2.0, 3.0),
            },
            BodyType::default(),
            None,
        );
        let cuboid = builder
            .shape()
            .as_cuboid()
            .expect("Created shape was not a cuboid");

        assert_eq!(cuboid.half_extents.x, 1.0);
        assert_eq!(cuboid.half_extents.y, 2.0);

        #[cfg(feature = "3d")]
        assert_eq!(cuboid.half_extents.z, 3.0);
    }

    #[test]
    fn build_capsule() {
        let builder = build_collider(
            Entity::new(0),
            &Body::Capsule {
                half_segment: 10.0,
                radius: 5.0,
            },
            BodyType::default(),
            None,
        );
        let capsule = builder
            .shape()
            .as_capsule()
            .expect("Created shape was not a capsule");

        assert_eq!(capsule.radius, 5.0);
        assert_eq!(capsule.segment.a.x, 0.0);
        assert_eq!(capsule.segment.b.x, 0.0);
        assert_eq!(capsule.segment.a.y, -10.0);
        assert_eq!(capsule.segment.b.y, 10.0);

        #[cfg(feature = "3d")]
        assert_eq!(capsule.segment.a.z, 0.0);
        #[cfg(feature = "3d")]
        assert_eq!(capsule.segment.b.z, 0.0);
    }
}

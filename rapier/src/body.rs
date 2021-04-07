use bevy::ecs::prelude::*;
use bevy::math::prelude::*;
use bevy::transform::prelude::*;
use fnv::FnvHashMap;

use heron_core::{Body, BodyType, PhysicMaterial, RotationConstraints, Velocity};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{
    BodyStatus, JointSet, RigidBodyBuilder, RigidBodyHandle, RigidBodySet,
};
use crate::rapier::geometry::{Collider, ColliderBuilder, ColliderSet};
use crate::rapier::math::Point;
use crate::BodyHandle;

pub(crate) type HandleMap = FnvHashMap<Entity, RigidBodyHandle>;

trait ColliderFactory {
    fn collider_builder(&self) -> ColliderBuilder;

    fn build(&self, entity: Entity, body_type: BodyType, material: PhysicMaterial) -> Collider {
        let mut collider_builder = self.collider_builder();
        collider_builder = collider_builder
            .user_data(entity.to_bits().into())
            .sensor(matches!(body_type, BodyType::Sensor))
            .restitution(material.restitution)
            .density(material.density);
        collider_builder.build()
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn create(
    mut commands: Commands,
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
            Option<&PhysicMaterial>,
            Option<&RotationConstraints>,
        ),
        Without<BodyHandle>,
    >,
) {
    for (entity, body, transform, body_type, velocity, material, rotation_constraints) in
        query.iter()
    {
        let body_type = body_type.cloned().unwrap_or_default();

        let mut builder = RigidBodyBuilder::new(body_status(body_type))
            .user_data(entity.to_bits().into())
            .position((transform.translation, transform.rotation).into_rapier());

        #[allow(unused_variables)]
        if let Some(RotationConstraints {
            allow_x,
            allow_y,
            allow_z,
        }) = rotation_constraints.cloned()
        {
            #[cfg(feature = "2d")]
            if !allow_z {
                builder = builder.lock_rotations();
            }
            #[cfg(feature = "3d")]
            {
                builder = builder.restrict_rotations(allow_x, allow_y, allow_z);
            }
        }

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
        let collider = body.build(entity, body_type, material.cloned().unwrap_or_default());

        let collider_handle = colliders.insert(collider, rigid_body, &mut bodies);
        handles.insert(entity, rigid_body);
        commands.insert(
            entity,
            BodyHandle {
                rigid_body,
                collider: collider_handle,
            },
        );
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn remove_bodies(
    mut commands: Commands,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
    changed: Query<
        '_,
        (Entity, &BodyHandle),
        Or<(
            Mutated<Body>,
            Changed<RotationConstraints>,
            Changed<BodyType>,
            Changed<PhysicMaterial>,
        )>,
    >,
    removed: Query<'_, (Entity, &BodyHandle), Without<RotationConstraints>>,
) {
    for (entity, handle) in changed.iter() {
        bodies.remove(handle.rigid_body, &mut colliders, &mut joints);
        commands.remove_one::<BodyHandle>(entity);
    }

    for entity in removed.removed::<RotationConstraints>() {
        if let Ok((entity, handle)) = removed.get(*entity) {
            bodies.remove(handle.rigid_body, &mut colliders, &mut joints);
            commands.remove_one::<BodyHandle>(entity);
        }
    }
}

pub(crate) fn update_rapier_status(
    mut bodies: ResMut<'_, RigidBodySet>,
    with_type_changed: Query<'_, (&BodyType, &BodyHandle), Changed<BodyType>>,
    without_type: Query<'_, &BodyHandle, Without<BodyType>>,
) {
    for (body_type, handle) in with_type_changed.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
            body.set_body_status(body_status(*body_type));
        }
    }

    for entity in without_type.removed::<BodyType>() {
        if let Some(body) = without_type
            .get(*entity)
            .ok()
            .and_then(|handle| bodies.get_mut(handle.rigid_body))
        {
            body.set_body_status(body_status(BodyType::default()));
        }
    }
}

pub(crate) fn update_rapier_position(
    mut bodies: ResMut<'_, RigidBodySet>,
    query: Query<'_, (&GlobalTransform, &BodyHandle), Mutated<GlobalTransform>>,
) {
    for (transform, handle) in query.iter() {
        if let Some(body) = bodies.get_mut(handle.rigid_body) {
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
        (
            Option<&mut Transform>,
            &mut GlobalTransform,
            &BodyHandle,
            Option<&BodyType>,
        ),
    >,
) {
    for (mut local, mut global, handle, body_type) in query.iter_mut() {
        if !body_type.cloned().unwrap_or_default().can_have_velocity() {
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
    mut commands: Commands,
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

impl ColliderFactory for Body {
    fn collider_builder(&self) -> ColliderBuilder {
        match self {
            Body::Sphere { radius } => ColliderBuilder::ball(*radius),
            Body::Capsule {
                half_segment: half_height,
                radius,
            } => ColliderBuilder::capsule_y(*half_height, *radius),
            Body::Cuboid { half_extends } => cuboid_builder(*half_extends),
            Body::ConvexHull { points } => convex_hull_builder(points.as_slice()),
        }
    }
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

#[inline]
fn convex_hull_builder(points: &[Vec3]) -> ColliderBuilder {
    let points: Vec<Point<f32>> = points.into_rapier();
    ColliderBuilder::convex_hull(points.as_slice()).expect("Failed to create convex-hull")
}

fn body_status(body_type: BodyType) -> BodyStatus {
    match body_type {
        BodyType::Dynamic => BodyStatus::Dynamic,
        BodyType::Static | BodyType::Sensor => BodyStatus::Static,
        BodyType::Kinematic => BodyStatus::Kinematic,
    }
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::*;

    #[test]
    fn build_sphere() {
        let collider = Body::Sphere { radius: 4.2 }.build(
            Entity::new(0),
            BodyType::default(),
            Default::default(),
        );

        let ball = collider
            .shape()
            .as_ball()
            .expect("Created shape was not a ball");
        assert_eq!(ball.radius, 4.2);
    }

    #[test]
    fn build_cuboid() {
        let collider = Body::Cuboid {
            half_extends: Vec3::new(1.0, 2.0, 3.0),
        }
        .build(Entity::new(0), BodyType::default(), Default::default());

        let cuboid = collider
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
        let collider = Body::Capsule {
            half_segment: 10.0,
            radius: 5.0,
        }
        .build(Entity::new(0), BodyType::default(), Default::default());

        let capsule = collider
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

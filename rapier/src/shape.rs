use bevy::prelude::*;

use heron_core::{CollisionShape, PhysicMaterial, RigidBody};

use crate::convert::IntoRapier;
use crate::rapier::dynamics::{RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{Collider, ColliderBuilder, ColliderHandle, ColliderSet};
use crate::rapier::math::Point;

pub(crate) fn create(
    mut commands: Commands<'_>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    body_handles: Query<
        '_,
        (
            Option<&RigidBody>,
            &RigidBodyHandle,
            Option<&PhysicMaterial>,
        ),
    >,
    shapes: Query<
        '_,
        (Entity, &CollisionShape, Option<&Parent>, Option<&Transform>),
        Without<ColliderHandle>,
    >,
) {
    for (entity, shape, _, _) in shapes.iter() {
        if let Ok((body, rigid_body_handle, material)) = body_handles.get(entity) {
            let collider = shape.build(
                entity,
                body.copied().unwrap_or_default(),
                material.copied().unwrap_or_default(),
            );

            commands.entity(entity).insert(colliders.insert(
                collider,
                *rigid_body_handle,
                &mut bodies,
            ));
        }
    }
}

trait ColliderFactory {
    fn collider_builder(&self) -> ColliderBuilder;

    fn build(&self, entity: Entity, body_type: RigidBody, material: PhysicMaterial) -> Collider {
        let mut collider_builder = self.collider_builder();
        collider_builder = collider_builder
            .user_data(entity.to_bits().into())
            .sensor(matches!(body_type, RigidBody::Sensor))
            .restitution(material.restitution)
            .density(material.density)
            .friction(material.friction);
        collider_builder.build()
    }
}

impl ColliderFactory for CollisionShape {
    fn collider_builder(&self) -> ColliderBuilder {
        match self {
            CollisionShape::Sphere { radius } => ColliderBuilder::ball(*radius),
            CollisionShape::Capsule {
                half_segment: half_height,
                radius,
            } => ColliderBuilder::capsule_y(*half_height, *radius),
            CollisionShape::Cuboid { half_extends } => cuboid_builder(*half_extends),
            CollisionShape::ConvexHull { points } => convex_hull_builder(points.as_slice()),
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

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::*;

    #[test]
    fn build_sphere() {
        let collider = CollisionShape::Sphere { radius: 4.2 }.build(
            Entity::new(0),
            RigidBody::default(),
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
        let collider = CollisionShape::Cuboid {
            half_extends: Vec3::new(1.0, 2.0, 3.0),
        }
        .build(Entity::new(0), RigidBody::default(), Default::default());

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
        let collider = CollisionShape::Capsule {
            half_segment: 10.0,
            radius: 5.0,
        }
        .build(Entity::new(0), RigidBody::default(), Default::default());

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

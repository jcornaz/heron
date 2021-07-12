use bevy::prelude::*;
use fnv::FnvHashMap;

use heron_core::{CollisionLayers, CollisionShape, PhysicMaterial, RigidBody, SensorShape};

use crate::convert::IntoRapier;
use crate::rapier::dynamics::{IslandManager, RigidBodyHandle, RigidBodySet};
use crate::rapier::geometry::{
    Collider, ColliderBuilder, ColliderHandle, ColliderSet, InteractionGroups,
};
use crate::rapier::math::Point;
use crate::rapier::pipeline::ActiveEvents;

pub(crate) type HandleMap = FnvHashMap<Entity, ColliderHandle>;

pub(crate) fn create(
    mut commands: Commands<'_>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut handles: ResMut<'_, HandleMap>,
    rigid_bodies: Query<'_, (&RigidBody, &RigidBodyHandle, Option<&PhysicMaterial>)>,
    collision_shapes: Query<
        '_,
        (
            Entity,
            &CollisionShape,
            Option<&Parent>,
            Option<&Transform>,
            Option<&CollisionLayers>,
            Option<&SensorShape>,
        ),
        Without<ColliderHandle>,
    >,
) {
    for (entity, shape, parent, transform, layers, sensor_flag) in collision_shapes.iter() {
        let collider = if let Ok((body, rigid_body_handle, material)) = rigid_bodies.get(entity) {
            Some((
                shape.build(
                    entity,
                    sensor_flag.is_some() || matches!(body, RigidBody::Sensor),
                    material,
                    None,
                    layers,
                ),
                rigid_body_handle,
            ))
        } else if let Some((body, rigid_body_handle, material)) =
            parent.and_then(|p| rigid_bodies.get(p.0).ok())
        {
            Some((
                shape.build(
                    entity,
                    sensor_flag.is_some() || matches!(body, RigidBody::Sensor),
                    material,
                    transform,
                    layers,
                ),
                rigid_body_handle,
            ))
        } else {
            None
        };

        if let Some((collider, rigid_body_handle)) = collider {
            let handle = colliders.insert_with_parent(collider, *rigid_body_handle, &mut bodies);
            commands.entity(entity).insert(handle);
            handles.insert(entity, handle);
        }
    }
}

pub(crate) fn update_position(
    mut colliders: ResMut<'_, ColliderSet>,
    query: Query<'_, (&Transform, &ColliderHandle), (Changed<Transform>, Without<RigidBody>)>,
) {
    for (transform, handle) in query.iter() {
        if let Some(collider) = colliders.get_mut(*handle) {
            collider
                .set_position_wrt_parent((transform.translation, transform.rotation).into_rapier())
        }
    }
}

pub(crate) fn update_collision_groups(
    mut colliders: ResMut<'_, ColliderSet>,
    query: Query<'_, (&CollisionLayers, &ColliderHandle), Changed<CollisionLayers>>,
) {
    for (layers, handle) in query.iter() {
        if let Some(collider) = colliders.get_mut(*handle) {
            collider.set_collision_groups(layers.into_rapier());
        }
    }
}

pub(crate) fn update_sensor_flag(
    mut colliders: ResMut<'_, ColliderSet>,
    query: Query<'_, &ColliderHandle, Changed<SensorShape>>,
) {
    for handle in query.iter() {
        if let Some(collider) = colliders.get_mut(*handle) {
            collider.set_sensor(true);
        }
    }
}

#[allow(clippy::cast_possible_truncation)]
pub(crate) fn remove_sensor_flag(
    bodies: Res<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    rigid_bodies: Query<'_, &RigidBody>,
    collider_handles: Query<'_, &ColliderHandle>,
    removed: RemovedComponents<'_, SensorShape>,
) {
    removed
        .iter()
        .filter_map(|e| collider_handles.get(e).ok())
        .for_each(|handle| {
            if let Some(collider) = colliders.get_mut(*handle) {
                let rigid_body = collider.parent().and_then(|parent| {
                    bodies
                        .get(parent)
                        .map(|b| Entity::from_bits(b.user_data as u64))
                        .and_then(|e| rigid_bodies.get(e).ok())
                });

                collider.set_sensor(matches!(rigid_body, Some(RigidBody::Sensor)));
            }
        });
}

pub(crate) fn reset_collision_groups(
    mut colliders: ResMut<'_, ColliderSet>,
    handles: Query<'_, &ColliderHandle>,
    removed: RemovedComponents<'_, CollisionLayers>,
) {
    removed
        .iter()
        .filter_map(|entity| handles.get(entity).ok())
        .for_each(|handle| {
            if let Some(collider) = colliders.get_mut(*handle) {
                collider.set_collision_groups(InteractionGroups::default());
            }
        });
}

pub(crate) fn remove_invalids_after_components_removed(
    mut commands: Commands<'_>,
    mut handles: ResMut<'_, HandleMap>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut islands: ResMut<'_, IslandManager>,
    mut colliders: ResMut<'_, ColliderSet>,
    shapes_removed: RemovedComponents<'_, CollisionShape>,
) {
    for entity in shapes_removed.iter() {
        if let Some(handle) = handles.remove(&entity) {
            colliders.remove(handle, &mut islands, &mut bodies, true);
            commands.entity(entity).remove::<ColliderHandle>();
        }
    }
}

#[allow(clippy::type_complexity)]
pub(crate) fn remove_invalids_after_component_changed(
    mut commands: Commands<'_>,
    mut handles: ResMut<'_, HandleMap>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut islands: ResMut<'_, IslandManager>,
    mut colliders: ResMut<'_, ColliderSet>,
    changed: Query<'_, (Entity, &ColliderHandle), Changed<CollisionShape>>,
) {
    for (entity, handle) in changed.iter() {
        colliders.remove(*handle, &mut islands, &mut bodies, true);
        commands.entity(entity).remove::<ColliderHandle>();
        handles.remove(&entity);
    }
}

pub(crate) trait ColliderFactory {
    fn collider_builder(&self) -> ColliderBuilder;

    fn build(
        &self,
        entity: Entity,
        is_sensor: bool,
        material: Option<&PhysicMaterial>,
        transform: Option<&Transform>,
        layers: Option<&CollisionLayers>,
    ) -> Collider {
        let mut builder = self
            .collider_builder()
            .user_data(entity.to_bits().into())
            .sensor(is_sensor);

        if let Some(material) = material {
            builder = builder
                .restitution(material.restitution)
                .density(material.density)
                .friction(material.friction);
        }

        if let Some(transform) = transform {
            builder = builder.position((transform.translation, transform.rotation).into_rapier());
        }

        if let Some(layers) = layers {
            builder = builder.collision_groups(layers.into_rapier());
        }

        builder.build()
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
            CollisionShape::HeightField { size, heights } => heightfield_builder(*size, &heights),
        }
        // General all types of collision events
        .active_events(ActiveEvents::all())
    }
}

#[inline]
#[cfg(dim2)]
fn cuboid_builder(half_extends: Vec3) -> ColliderBuilder {
    ColliderBuilder::cuboid(half_extends.x, half_extends.y)
}

#[inline]
#[cfg(dim3)]
fn cuboid_builder(half_extends: Vec3) -> ColliderBuilder {
    ColliderBuilder::cuboid(half_extends.x, half_extends.y, half_extends.z)
}

#[inline]
fn convex_hull_builder(points: &[Vec3]) -> ColliderBuilder {
    let points: Vec<Point<f32>> = points.into_rapier();
    ColliderBuilder::convex_hull(points.as_slice()).expect("Failed to create convex-hull")
}

#[inline]
#[cfg(dim2)]
#[allow(clippy::cast_precision_loss)]
fn heightfield_builder(size: Vec2, heights: &[Vec<f32>]) -> ColliderBuilder {
    let len = heights.get(0).map(Vec::len).unwrap_or_default();
    ColliderBuilder::heightfield(
        crate::rapier::na::DVector::from_iterator(len, heights.iter().flatten().take(len).cloned()),
        crate::rapier::na::Vector2::new(size.x, 1.0),
    )
}

#[inline]
#[cfg(dim3)]
#[allow(clippy::cast_precision_loss)]
fn heightfield_builder(size: Vec2, heights: &[Vec<f32>]) -> ColliderBuilder {
    let nrows = heights.len();
    let ncols = heights.get(0).map(Vec::len).unwrap_or_default();
    ColliderBuilder::heightfield(
        crate::rapier::na::DMatrix::from_iterator(nrows, ncols, heights.iter().flatten().cloned()),
        crate::rapier::na::Vector3::new(size.x, 1.0, size.y),
    )
}

#[cfg(test)]
mod tests {
    use bevy::math::Vec3;

    use super::*;

    #[test]
    fn build_sphere() {
        let collider = CollisionShape::Sphere { radius: 4.2 }
            .collider_builder()
            .build();

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
        .collider_builder()
        .build();

        let cuboid = collider
            .shape()
            .as_cuboid()
            .expect("Created shape was not a cuboid");

        assert_eq!(cuboid.half_extents.x, 1.0);
        assert_eq!(cuboid.half_extents.y, 2.0);

        #[cfg(dim3)]
        assert_eq!(cuboid.half_extents.z, 3.0);
    }

    #[test]
    fn build_capsule() {
        let collider = CollisionShape::Capsule {
            half_segment: 10.0,
            radius: 5.0,
        }
        .collider_builder()
        .build();

        let capsule = collider
            .shape()
            .as_capsule()
            .expect("Created shape was not a capsule");

        assert_eq!(capsule.radius, 5.0);
        assert_eq!(capsule.segment.a.x, 0.0);
        assert_eq!(capsule.segment.b.x, 0.0);
        assert_eq!(capsule.segment.a.y, -10.0);
        assert_eq!(capsule.segment.b.y, 10.0);

        #[cfg(dim3)]
        assert_eq!(capsule.segment.a.z, 0.0);
        #[cfg(dim3)]
        assert_eq!(capsule.segment.b.z, 0.0);
    }

    #[test]
    #[cfg(any(dim2, dim3))]
    fn build_heightfield() {
        let collider = CollisionShape::HeightField {
            size: Vec2::new(2.0, 1.0),
            heights: vec![vec![1.0, 2.0, 3.0], vec![4.0, 5.0, 6.0]],
        }
        .collider_builder()
        .build();

        let field = collider
            .shape()
            .as_heightfield()
            .expect("Created shape was not a height field");

        #[cfg(dim2)]
        {
            assert_eq!(field.num_cells(), 2); // Three points = 2 segments
            assert_eq!(field.cell_width(), 1.0);
        }

        #[cfg(dim3)]
        {
            assert_eq!(field.nrows(), 1);
            assert_eq!(field.ncols(), 2);
            assert_eq!(
                field.scale(),
                &crate::rapier::na::Vector3::new(2.0, 1.0, 1.0)
            );
            assert_eq!(field.cell_height(), 1.0);
            assert_eq!(field.cell_width(), 1.0);
        }
    }
}

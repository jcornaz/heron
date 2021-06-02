use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::RectangleOrigin;

use heron_core::CollisionShape;
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::geometry::{ColliderHandle, ColliderSet, Shape};

use super::*;

pub(crate) fn systems() -> SystemSet {
    SystemSet::new()
        .with_system(dim2::delete_debug_sprite.system())
        .with_system(dim2::replace_debug_sprite.system())
        .with_system(dim2::create_debug_sprites.system())
}

fn create_debug_sprites(
    mut commands: Commands<'_>,
    colliders: Res<'_, ColliderSet>,
    query: Query<
        '_,
        (Entity, &CollisionShape, &ColliderHandle, &GlobalTransform),
        Without<HasDebug>,
    >,
    debug_color: Res<'_, DebugColor>,
) {
    for (entity, body, handle, transform) in query.iter() {
        if let Some(collider) = colliders.get(*handle) {
            commands
                .entity(entity)
                .with_children(|builder| {
                    builder
                        .spawn_bundle(create_shape(
                            body,
                            collider.shape(),
                            (*debug_color).into(),
                            *transform,
                        ))
                        .insert(IsDebug(entity));
                })
                .insert(HasDebug);
        }
    }
}

fn replace_debug_sprite(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    colliders: Res<'_, ColliderSet>,
    debug_color: Res<'_, DebugColor>,
    query: Query<
        '_,
        (Entity, &CollisionShape, &ColliderHandle, &GlobalTransform),
        (With<HasDebug>, Changed<CollisionShape>),
    >,
) {
    for (parent_entity, body, handle, transform) in query.iter() {
        if let (Some(debug_entity), Some(collider)) =
            (map.remove(&parent_entity), colliders.get(*handle))
        {
            commands.entity(debug_entity).despawn();
            commands.entity(parent_entity).with_children(|builder| {
                builder
                    .spawn_bundle(create_shape(
                        body,
                        collider.shape(),
                        (*debug_color).into(),
                        *transform,
                    ))
                    .insert(IsDebug(parent_entity));
            });
        }
    }
}

fn delete_debug_sprite(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    removed_bodies: RemovedComponents<'_, CollisionShape>,
) {
    for parent_entity in removed_bodies.iter() {
        if let Some(debug_entity) = map.remove(&parent_entity) {
            commands.entity(debug_entity).despawn();
        }
    }
}

fn create_shape(
    body: &CollisionShape,
    shape: &dyn Shape,
    color: Color,
    transform: GlobalTransform,
) -> ShapeBundle {
    base_builder(body, shape).build(
        ShapeColors::new(color),
        DrawMode::Fill(FillOptions::default()),
        Transform {
            translation: Vec3::Z,
            scale: transform.scale.recip(),
            ..Default::default()
        },
    )
}

fn base_builder(body: &CollisionShape, shape: &dyn Shape) -> GeometryBuilder {
    let mut builder = GeometryBuilder::new();

    match body {
        CollisionShape::Sphere { radius } => {
            builder.add(&shapes::Circle {
                radius: *radius,
                center: Vec2::ZERO,
            });
        }
        CollisionShape::Capsule {
            half_segment,
            radius,
        } => {
            let half_segment = *half_segment;
            let radius = *radius;
            let mut path = PathBuilder::new();
            path.move_to(Vec2::new(-radius, half_segment));
            path.arc(
                Vec2::new(0.0, half_segment),
                Vec2::new(radius, radius),
                -PI,
                0.0,
            );
            path.line_to(Vec2::new(radius, -half_segment));
            path.arc(
                Vec2::new(0.0, -half_segment),
                Vec2::new(radius, radius),
                -PI,
                0.0,
            );
            path.close();
            builder.add(&path.build());
        }
        CollisionShape::Cuboid { half_extends } => {
            builder.add(&shapes::Rectangle {
                origin: RectangleOrigin::Center,
                width: 2.0 * half_extends.x,
                height: 2.0 * half_extends.y,
            });
        }
        CollisionShape::ConvexHull { .. } => {
            if let Some(polygon) = shape.as_convex_polygon() {
                builder.add(&shapes::Polygon {
                    points: polygon.points().into_bevy(),
                    closed: true,
                });
            }
        }
        CollisionShape::HeightField { size, heights } => {
            if let Some(heights) = heights.get(0) {
                let mut points: Vec<Vec2> = Vec::with_capacity(heights.len() + 2);
                let mut min_y = f32::MAX;

                heights
                    .iter()
                    .enumerate()
                    .map(|(i, p)| Vec2::new(i as f32, *p))
                    .for_each(|p| {
                        if p.y < min_y {
                            min_y = p.y;
                        }
                        points.push(p);
                    });

                #[allow(clippy::cast_precision_loss)]
                points.push(Vec2::new(size.x, min_y));
                points.push(Vec2::new(0.0, min_y));

                builder.add(&shapes::Polygon {
                    points,
                    closed: true,
                });
            }
        }
    };

    builder
}

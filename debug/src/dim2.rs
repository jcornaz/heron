use std::f32::consts::PI;

use bevy::asset::prelude::*;
use bevy::ecs::prelude::*;
use bevy::math::{Vec2, Vec3};
use bevy::sprite::prelude::*;
use bevy::transform::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::RectangleOrigin;

use heron_core::Body;
use heron_rapier::convert::IntoBevy;
use heron_rapier::rapier::geometry::{ColliderSet, Shape};
use heron_rapier::BodyHandle;

use super::*;

pub(crate) fn create_debug_sprites(
    mut commands: Commands,
    colliders: Res<'_, ColliderSet>,
    query: Query<'_, (Entity, &Body, &BodyHandle, &GlobalTransform), Without<HasDebug>>,
    debug_color: Res<'_, DebugColor>,
) {
    for (entity, body, handle, transform) in query.iter() {
        if let Some(collider) = colliders.get(handle.collider()) {
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

pub(crate) fn replace_debug_sprite(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    colliders: Res<'_, ColliderSet>,
    debug_color: Res<'_, DebugColor>,
    query: Query<
        '_,
        (Entity, &Body, &BodyHandle, &GlobalTransform),
        (With<HasDebug>, Changed<Body>),
    >,
) {
    for (parent_entity, body, handle, transform) in query.iter() {
        if let (Some(debug_entity), Some(collider)) =
            (map.remove(&parent_entity), colliders.get(handle.collider()))
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

pub(crate) fn delete_debug_sprite(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    removed_bodies: RemovedComponents<'_, Body>,
) {
    for parent_entity in removed_bodies.iter() {
        if let Some(debug_entity) = map.remove(&parent_entity) {
            commands.entity(debug_entity).despawn();
        }
    }
}

fn create_shape(
    body: &Body,
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

fn base_builder(body: &Body, shape: &dyn Shape) -> GeometryBuilder {
    let mut builder = GeometryBuilder::new();

    match body {
        Body::Sphere { radius } => {
            builder.add(&shapes::Circle {
                radius: *radius,
                center: Vec2::ZERO,
            });
        }
        Body::Capsule {
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
        Body::Cuboid { half_extends } => {
            builder.add(&shapes::Rectangle {
                origin: RectangleOrigin::Center,
                width: 2.0 * half_extends.x,
                height: 2.0 * half_extends.y,
            });
        }
        Body::ConvexHull { .. } => {
            if let Some(polygon) = shape.as_convex_polygon() {
                builder.add(&shapes::Polygon {
                    points: polygon.points().into_bevy(),
                    closed: true,
                });
            }
        }
    };

    builder
}

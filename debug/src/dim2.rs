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
    debug_mat: Res<'_, DebugMaterial>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (entity, body, handle, transform) in query.iter() {
        if let Some(collider) = colliders.get(handle.collider()) {
            commands.set_current_entity(entity);
            commands
                .with_children(|builder| {
                    builder
                        .spawn(create_shape(
                            body,
                            collider.shape(),
                            material.clone(),
                            *transform,
                        ))
                        .with(IsDebug(entity));
                })
                .with(HasDebug);
        }
    }
}

pub(crate) fn replace_debug_sprite(
    mut commands: Commands,
    mut map: ResMut<'_, DebugEntityMap>,
    colliders: Res<'_, ColliderSet>,
    query: Query<
        '_,
        (Entity, &Body, &BodyHandle, &GlobalTransform),
        (With<HasDebug>, Mutated<Body>),
    >,
    debug_mat: Res<'_, DebugMaterial>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (parent_entity, body, handle, transform) in query.iter() {
        if let (Some(debug_entity), Some(collider)) =
            (map.remove(&parent_entity), colliders.get(handle.collider()))
        {
            commands.despawn(debug_entity);
            commands.set_current_entity(parent_entity);
            commands.with_children(|builder| {
                builder
                    .spawn(create_shape(
                        body,
                        collider.shape(),
                        material.clone(),
                        *transform,
                    ))
                    .with(IsDebug(parent_entity));
            });
        }
    }
}

pub(crate) fn delete_debug_sprite(
    mut commands: Commands,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, Entity, (With<HasDebug>, Without<Body>)>,
) {
    for parent_entity in query.removed::<Body>() {
        if let Some(debug_entity) = map.remove(&parent_entity) {
            commands.despawn(debug_entity);
        }
    }
}

fn create_shape(
    body: &Body,
    shape: &dyn Shape,
    material: Handle<ColorMaterial>,
    transform: GlobalTransform,
) -> ShapeBundle {
    base_builder(body, shape).build(
        material,
        TessellationMode::Fill(FillOptions::default()),
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
                center: Vec2::zero(),
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

use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::{Vec2, Vec3};
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_sprite::prelude::*;
use bevy_transform::prelude::*;

use heron_core::Body;

use super::*;
use std::f32::consts::PI;

pub(crate) fn create_debug_sprites(
    commands: &mut Commands,
    query: Query<'_, (Entity, &Body, &GlobalTransform), Without<HasDebug>>,
    debug_mat: Res<'_, DebugMaterial>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (entity, body, transform) in query.iter() {
        commands.set_current_entity(entity);
        commands
            .with_children(|builder| {
                builder
                    .spawn(create_shape(body, material.clone(), *transform))
                    .with(IsDebug(entity));
            })
            .with(HasDebug);
    }
}

pub(crate) fn replace_debug_sprite(
    commands: &mut Commands,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, (Entity, &Body, &GlobalTransform), (With<HasDebug>, Mutated<Body>)>,
    debug_mat: Res<'_, DebugMaterial>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (parent_entity, body, transform) in query.iter() {
        if let Some(debug_entity) = map.remove(&parent_entity) {
            commands.despawn(debug_entity);
            commands.set_current_entity(parent_entity);
            commands.with_children(|builder| {
                builder
                    .spawn(create_shape(body, material.clone(), *transform))
                    .with(IsDebug(parent_entity));
            });
        }
    }
}

pub(crate) fn delete_debug_sprite(
    commands: &mut Commands,
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
    material: Handle<ColorMaterial>,
    transform: GlobalTransform,
) -> ShapeBundle {
    base_builder(body).build(
        material,
        TessellationMode::Fill(FillOptions::default()),
        Transform {
            translation: Vec3::unit_z(),
            scale: transform.scale.recip(),
            ..Default::default()
        },
    )
}

fn base_builder(body: &Body) -> GeometryBuilder {
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
    };

    builder
}

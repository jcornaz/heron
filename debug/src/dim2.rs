use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_prototype_lyon::entity::ShapeBundle;
use bevy_prototype_lyon::prelude::*;
use bevy_prototype_lyon::shapes::RectangleOrigin;

use heron_core::CollisionShape;
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
    let mut builder = GeometryBuilder::new();

    base_builder(
        &mut builder,
        body,
        shape,
        Default::default(),
        Default::default(),
    );

    builder.build(
        ShapeColors::new(color),
        DrawMode::Fill(FillOptions::default()),
        Transform {
            translation: Vec3::Z,
            scale: transform.scale.recip(),
            ..Default::default()
        },
    )
}

fn base_builder(
    builder: &mut GeometryBuilder,
    body: &CollisionShape,
    shape: &dyn Shape,
    translation: Vec2,
    rotation: Quat,
) {
    if rotation != Quat::default() {
        bevy::log::warn!("Debug rendering of rotated compound sub-shapes is not implemented yet");
    }
    let matrix = Mat3::from_scale_angle_translation(Vec2::new(1., 1.), 0., translation);
    let get_point = |x: f32, y: f32| -> Vec2 { matrix.transform_point2(Vec2::new(x, y)) };

    match body {
        CollisionShape::Sphere { radius } => {
            builder.add(&shapes::Circle {
                radius: *radius,
                center: get_point(0., 0.),
            });
        }
        CollisionShape::Capsule {
            half_segment,
            radius,
        } => {
            let half_segment = *half_segment;
            let radius = *radius;
            let mut path = PathBuilder::new();
            path.move_to(get_point(-radius, half_segment));
            path.arc(
                get_point(0.0, half_segment),
                get_point(radius, radius),
                -PI,
                0.0,
            );
            path.line_to(get_point(radius, -half_segment));
            path.arc(
                get_point(0.0, -half_segment),
                get_point(radius, radius),
                -PI,
                0.0,
            );
            path.close();
            builder.add(&path.build());
        }
        CollisionShape::Cuboid {
            half_extends,
            border_radius,
        } => {
            // bevy_prototype_lyon doesn't have rounded rectangles yet so we implement our own
            use lyon_path::{
                builder::BorderRadii,
                math::{Point, Rect, Size},
                path::Builder,
                traits::PathBuilder,
            };
            struct RoundedRectangle {
                width: f32,
                height: f32,
                radius: f32,
                origin: Vec2,
            }
            impl Geometry for RoundedRectangle {
                fn add_geometry(&self, b: &mut Builder) {
                    let real_width = self.width + self.radius * 2.0;
                    let real_height = self.height + self.radius * 2.0;

                    b.add_rounded_rectangle(
                        &Rect::new(
                            Point::new(self.origin.x, self.origin.y),
                            Size::new(real_width, real_height),
                        ),
                        &BorderRadii {
                            top_left: self.radius,
                            top_right: self.radius,
                            bottom_left: self.radius,
                            bottom_right: self.radius,
                        },
                        lyon_path::Winding::Positive,
                    );
                }
            }

            if let Some(radius) = border_radius {
                builder.add(&RoundedRectangle {
                    width: 2.0 * half_extends.x,
                    height: 2.0 * half_extends.y,
                    radius: *radius,
                    origin: get_point(0., 0.),
                });
            } else {
                builder.add(&shapes::Rectangle {
                    origin: RectangleOrigin::CustomCenter(get_point(0., 0.)),
                    width: 2.0 * half_extends.x,
                    height: 2.0 * half_extends.y,
                });
            }
        }
        CollisionShape::ConvexHull { .. } => {
            if let Some(polygon) = shape.as_convex_polygon() {
                builder.add(&shapes::Polygon {
                    points: polygon
                        .points()
                        .iter()
                        .map(|p| get_point(p.x, p.y))
                        .collect(),
                    closed: true,
                });

            // TODO: Implement better rounded convex hull renderer. Currently our strategy is to
            // render a circle at each point on the hull to give an impression of what the
            // border radius adds to the hull, but we don't currently fill in the empty space
            // around the edges of the polygon that are also taken up by the border radius.
            } else if let Some(polygon) = shape.as_round_convex_polygon() {
                for point in polygon.base_shape.points() {
                    builder.add(&shapes::Circle {
                        radius: polygon.border_radius,
                        center: get_point(point.x, point.y),
                    });
                }

                builder.add(&shapes::Polygon {
                    points: polygon
                        .base_shape
                        .points()
                        .iter()
                        .map(|p| get_point(p.x, p.y))
                        .collect(),
                    closed: true,
                });
            }
        }
        CollisionShape::HeightField { size, heights } => {
            if let Some(heights) = heights.get(0) {
                let mut points: Vec<Vec2> = Vec::with_capacity(heights.len() + 2);
                let mut min_y = f32::MAX;
                let half_size = size.x * 0.5;
                let len = (heights.len() - 1) as f32;

                heights
                    .iter()
                    .enumerate()
                    .map(|(i, p)| Vec2::new((i as f32) * size.x / len - half_size, *p))
                    .for_each(|p| {
                        if p.y < min_y {
                            min_y = p.y;
                        }
                        points.push(p);
                    });

                points.push(Vec2::new(half_size, min_y));
                points.push(Vec2::new(-half_size, min_y));

                builder.add(&shapes::Polygon {
                    points,
                    closed: true,
                });
            }
        }
        CollisionShape::Compound(shapes) => {
            for (sub_shape, (_, rapier_shape)) in
                shapes.iter().zip(shape.as_compound().unwrap().shapes())
            {
                base_builder(
                    builder,
                    &sub_shape.shape,
                    rapier_shape.as_ref(),
                    Vec2::new(sub_shape.translation.x, sub_shape.translation.y),
                    sub_shape.rotation,
                );
            }
        }
    };
}

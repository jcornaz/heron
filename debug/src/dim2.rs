use bevy::prelude::*;
use bevy_prototype_lyon::{entity::ShapeBundle, prelude::*};
use lyon_path::{
    builder::BorderRadii,
    math::{Point, Rect, Size},
    path::Builder,
    traits::PathBuilder,
};

use heron_core::{CollisionShape, RigidBody, SensorShape};
use heron_rapier::{
    convert::IntoBevy,
    rapier2d::geometry::{ColliderSet, Shape},
    ColliderHandle,
};

use super::*;

pub(crate) fn systems() -> SystemSet {
    SystemSet::new()
        .with_system(delete_debug_sprite.system())
        .with_system(replace_debug_sprite.system())
        .with_system(create_debug_sprites.system())
}

fn create_debug_sprites(
    mut commands: Commands<'_, '_>,
    colliders: Res<'_, ColliderSet>,
    query: Query<
        '_,
        '_,
        (
            Entity,
            &CollisionShape,
            &ColliderHandle,
            &GlobalTransform,
            Option<&RigidBody>,
            Option<&SensorShape>,
        ),
        Without<HasDebug>,
    >,
    debug_color: Res<'_, DebugColor>,
) {
    for (entity, body, handle, transform, rigid_body_option, sensor_option) in query.iter() {
        if let Some(collider) = colliders.get(**handle) {
            commands
                .entity(entity)
                .with_children(|builder| {
                    builder
                        .spawn_bundle(create_shape(
                            body,
                            collider.shape(),
                            debug_color
                                .for_collider_type(rigid_body_option, sensor_option.is_some()),
                            *transform,
                        ))
                        .insert(IsDebug(entity));
                })
                .insert(HasDebug);
        }
    }
}

fn replace_debug_sprite(
    mut commands: Commands<'_, '_>,
    mut map: ResMut<'_, DebugEntityMap>,
    colliders: Res<'_, ColliderSet>,
    debug_color: Res<'_, DebugColor>,
    query: Query<
        '_,
        '_,
        (
            Entity,
            &CollisionShape,
            &ColliderHandle,
            &GlobalTransform,
            Option<&RigidBody>,
            Option<&SensorShape>,
        ),
        (With<HasDebug>, Changed<CollisionShape>),
    >,
) {
    for (parent_entity, body, handle, transform, rigid_body_option, sensor_option) in query.iter() {
        if let (Some(debug_entity), Some(collider)) =
            (map.remove(&parent_entity), colliders.get(**handle))
        {
            commands.entity(debug_entity).despawn();
            commands.entity(parent_entity).with_children(|builder| {
                builder
                    .spawn_bundle(create_shape(
                        body,
                        collider.shape(),
                        debug_color.for_collider_type(rigid_body_option, sensor_option.is_some()),
                        *transform,
                    ))
                    .insert(IsDebug(parent_entity));
            });
        }
    }
}

fn delete_debug_sprite(
    mut commands: Commands<'_, '_>,
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
        DrawMode::Fill(FillMode {
            color,
            options: FillOptions::default(),
        }),
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
            builder = builder.add(&shapes::Circle {
                radius: *radius,
                center: Vec2::ZERO,
            });
        }
        CollisionShape::Capsule {
            half_segment,
            radius,
        } => {
            builder = builder.add(&Capsule {
                half_segment: *half_segment,
                radius: *radius,
            });
        }
        CollisionShape::Cuboid {
            half_extends,
            border_radius,
        } => {
            struct RoundedRectangle {
                width: f32,
                height: f32,
                radius: f32,
            }
            impl Geometry for RoundedRectangle {
                fn add_geometry(&self, b: &mut Builder) {
                    let real_width = self.width + self.radius * 2.0;
                    let real_height = self.height + self.radius * 2.0;
                    b.add_rounded_rectangle(
                        &Rect::new(
                            Point::new(-real_width / 2.0, -real_height / 2.0),
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

            builder = builder.add(&RoundedRectangle {
                width: 2.0 * half_extends.x,
                height: 2.0 * half_extends.y,
                radius: border_radius.unwrap_or(0.0),
            });
        }
        CollisionShape::ConvexHull { .. } => {
            if let Some(polygon) = shape.as_convex_polygon() {
                builder = builder.add(&shapes::Polygon {
                    points: polygon.points().into_bevy(),
                    closed: true,
                });

            // TODO: Implement better rounded convex hull renderer. Currently our strategy is to
            // render a circle at each point on the hull to give an impression of what the
            // border radius adds to the hull, but we don't currently fill in the empty space
            // around the edges of the polygon that are also taken up by the border radius.
            } else if let Some(polygon) = shape.as_round_convex_polygon() {
                for point in polygon.base_shape.points() {
                    builder = builder.add(&shapes::Circle {
                        radius: polygon.border_radius,
                        center: point.into_bevy(),
                    });
                }

                builder = builder.add(&shapes::Polygon {
                    points: polygon.base_shape.points().into_bevy(),
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

                builder = builder.add(&shapes::Polygon {
                    points,
                    closed: true,
                });
            }
        }
        any_other => {
            warn!(
                "Debug render for this shape {:?} is unimplemented",
                any_other
            );
        }
    };

    builder
}

struct Capsule {
    half_segment: f32,
    radius: f32,
}

impl Geometry for Capsule {
    fn add_geometry(&self, path: &mut Builder) {
        let ctrl_height = self.radius * (4. / 3.);
        path.begin(Point::new(-self.radius, self.half_segment));
        path.cubic_bezier_to(
            Point::new(-self.radius, self.half_segment + ctrl_height),
            Point::new(self.radius, self.half_segment + ctrl_height),
            Point::new(self.radius, self.half_segment),
        );
        path.line_to(Point::new(self.radius, -self.half_segment));
        path.cubic_bezier_to(
            Point::new(self.radius, -self.half_segment - ctrl_height),
            Point::new(-self.radius, -self.half_segment - ctrl_height),
            Point::new(-self.radius, -self.half_segment),
        );
        path.end(true);
    }
}

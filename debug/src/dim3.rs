use bevy::prelude::*;
use bevy_prototype_debug_lines::DebugLines;

use heron_core::{CollisionShape, RigidBody, SensorShape};

use crate::shape3d_wireframe::{
    add_capsule, add_cone, add_convex_hull, add_cuboid, add_cylinder, add_height_field,
    add_rounded_cuboid, add_sphere,
};

use super::DebugColor;

fn add_shape_outlines(
    shapes: Query<
        '_,
        '_,
        (
            &CollisionShape,
            &GlobalTransform,
            Option<&RigidBody>,
            Option<&SensorShape>,
        ),
    >,
    color: Res<'_, DebugColor>,
    mut lines: ResMut<'_, DebugLines>,
) {
    for (shape, trans, rigid_body_option, sensor_option) in shapes.iter() {
        let origin = trans.translation;
        let orient = trans.rotation;
        let color = color.for_collider_type(rigid_body_option, sensor_option.is_some());
        match shape {
            CollisionShape::Cuboid {
                half_extends,
                border_radius,
            } => match border_radius {
                Some(bevel) => {
                    add_rounded_cuboid(origin, orient, *half_extends, *bevel, color, &mut lines);
                }
                None => {
                    add_cuboid(origin, orient, *half_extends, color, &mut lines);
                }
            },
            CollisionShape::Sphere { radius } => {
                add_sphere(origin, orient, *radius, color, &mut lines);
            }
            CollisionShape::Capsule {
                half_segment,
                radius,
            } => add_capsule(origin, orient, *half_segment, *radius, color, &mut lines),
            CollisionShape::ConvexHull {
                points,
                border_radius: _,
            } => {
                // NOTE: won't work with ConvexHull with border_radius set,
                // absolutely no idea how to handle it here
                add_convex_hull(origin, orient, points, color, &mut lines);
            }
            CollisionShape::HeightField { size, heights } => {
                add_height_field(origin, orient, *size, heights, color, &mut lines);
            }
            CollisionShape::Cone {
                half_height,
                radius,
            } => {
                add_cone(origin, orient, *half_height, *radius, color, &mut lines);
            }
            CollisionShape::Cylinder {
                half_height,
                radius,
            } => {
                add_cylinder(origin, orient, *half_height, *radius, color, &mut lines);
            }
            any_other => {
                warn!(
                    "Debug render for this shape {:?} is unimplemented",
                    any_other
                );
            }
        }
    }
}

pub(crate) fn systems() -> SystemSet {
    SystemSet::new().with_system(add_shape_outlines.system())
}

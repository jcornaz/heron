use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    math::{Quat, Vec2, Vec3},
    render::color::Color,
};
use bevy_prototype_debug_lines::DebugLines;
use heron_rapier::convert::{IntoBevy, IntoRapier};
use heron_rapier::rapier3d::parry::transformation::convex_hull;

/// Vertex of a cuboid centered on origin
fn cuboid_vertex(half_length: Vec3) -> [Vec3; 8] {
    let x = half_length.x;
    let y = half_length.y;
    let z = half_length.z;
    let v3 = Vec3::new;
    [
        v3(x, y, z),
        v3(-x, y, z),
        v3(-x, y, -z),
        v3(x, y, -z),
        v3(x, -y, -z),
        v3(x, -y, z),
        v3(-x, -y, z),
        v3(-x, -y, -z),
    ]
}
const CUBOID_EDGES: [(usize, usize); 12] = [
    (0, 1),
    (1, 2),
    (2, 3),
    (3, 0),
    (4, 5),
    (5, 6),
    (6, 7),
    (7, 4),
    (0, 5),
    (1, 6),
    (2, 7),
    (3, 4),
];
// For the rounded corners: the direction in which you need to move
// an edge so that it is located where the edge really is after adding the
// radius to the cuboid
fn cuboid_edge_bevel_mods(index: usize) -> (Vec3, Vec3) {
    let x = Vec3::X;
    let y = Vec3::Y;
    let z = Vec3::Z;
    let dirs = [
        (z, y),
        (-x, y),
        (-z, y),
        (x, y),
        (x, -y),
        (z, -y),
        (-x, -y),
        (-z, -y),
        (x, z),
        (-x, z),
        (-x, -z),
        (x, -z),
    ];
    dirs[index]
}

// Picture picking a rounded corner and fitting it to a corner of a cube. You
// then rotate it to fit the next corner, each one after the other. The
// sequence of corners is specified by `cuboid_vertex()`, this returns the
// rotations you need to execute at each step.
fn cuboid_corner_bevel_rotation() -> [Quat; 7] {
    let y_rot = Quat::from_rotation_y;
    let z_rot = Quat::from_rotation_z;
    let x_rot = Quat::from_rotation_x;
    [
        y_rot(-FRAC_PI_2),
        y_rot(-FRAC_PI_2),
        y_rot(-FRAC_PI_2),
        z_rot(-FRAC_PI_2),
        x_rot(FRAC_PI_2),
        x_rot(FRAC_PI_2),
        x_rot(FRAC_PI_2),
    ]
}

fn add_rounded_corner(
    origin: Vec3,
    orient: Quat,
    radius: f32,
    color: Color,
    lines: &mut DebugLines,
) {
    let x_spin = Quat::from_rotation_x(FRAC_PI_2);
    let y_spin = Quat::from_rotation_y(-FRAC_PI_2);
    add_quartercircle(origin, orient * y_spin, radius, color, lines);
    add_quartercircle(origin, orient * x_spin, radius, color, lines);
    add_quartercircle(origin, orient, radius, color, lines);
}
pub(crate) fn add_rounded_cuboid(
    origin: Vec3,
    orient: Quat,
    half_length: Vec3,
    radius: f32,
    color: Color,
    lines: &mut DebugLines,
) {
    let verts = cuboid_vertex(half_length);
    let change_dir = cuboid_corner_bevel_rotation();
    let mut dir = Quat::IDENTITY;
    for i in 0..verts.len() {
        let corner_origin = origin + orient.mul_vec3(verts[i]);
        add_rounded_corner(corner_origin, orient * dir, radius, color, lines);
        dir = dir * change_dir[i % change_dir.len()];
    }
    let bevel_edges_points = |index, p| {
        let (p0, p1) = cuboid_edge_bevel_mods(index);
        let p0 = origin + orient.mul_vec3(p + p0 * radius);
        let p1 = origin + orient.mul_vec3(p + p1 * radius);
        (p0, p1)
    };
    for (i, &(p0, p1)) in CUBOID_EDGES.iter().enumerate() {
        let (p00, p01) = bevel_edges_points(i, verts[p0]);
        let (p10, p11) = bevel_edges_points(i, verts[p1]);
        lines.line_colored(p00, p10, 0.0, color);
        lines.line_colored(p01, p11, 0.0, color);
    }
}
pub(crate) fn add_cuboid(
    origin: Vec3,
    orient: Quat,
    half_length: Vec3,
    color: Color,
    lines: &mut DebugLines,
) {
    let verts = cuboid_vertex(half_length);
    for &(p0, p1) in &CUBOID_EDGES {
        let p0 = origin + orient.mul_vec3(verts[p0]);
        let p1 = origin + orient.mul_vec3(verts[p1]);
        lines.line_colored(p0, p1, 0.0, color);
    }
}
fn add_quartercircle(
    origin: Vec3,
    orient: Quat,
    radius: f32,
    color: Color,
    lines: &mut DebugLines,
) {
    let quarter_circle_segments = 4;
    let angle = FRAC_PI_2 / quarter_circle_segments as f32;
    let mut current_point = orient.mul_vec3(Vec3::X * radius);
    let direction = Quat::from_axis_angle(orient.mul_vec3(Vec3::Z), angle);
    for _ in 0..quarter_circle_segments {
        let next_point = direction.mul_vec3(current_point);
        lines.line_colored(origin + current_point, origin + next_point, 0.0, color);
        current_point = next_point;
    }
}
fn add_semicircle(origin: Vec3, orient: Quat, radius: f32, color: Color, lines: &mut DebugLines) {
    let x_rotate = Quat::from_rotation_y(PI);
    add_quartercircle(origin, orient, radius, color, lines);
    add_quartercircle(origin, orient * x_rotate, radius, color, lines);
}
fn add_circle(origin: Vec3, orient: Quat, radius: f32, color: Color, lines: &mut DebugLines) {
    let x_rotate = Quat::from_rotation_x(PI);
    add_semicircle(origin, orient, radius, color, lines);
    add_semicircle(origin, orient * x_rotate, radius, color, lines);
}
pub(crate) fn add_sphere(
    origin: Vec3,
    orient: Quat,
    radius: f32,
    color: Color,
    lines: &mut DebugLines,
) {
    let x_rotate = Quat::from_rotation_x(FRAC_PI_2);
    let y_rotate = Quat::from_rotation_y(FRAC_PI_2);
    add_circle(origin, orient, radius, color, lines);
    add_circle(origin, orient * x_rotate, radius, color, lines);
    add_circle(origin, orient * y_rotate, radius, color, lines);
}
pub(crate) fn add_capsule(
    origin: Vec3,
    orient: Quat,
    half_segment: f32,
    radius: f32,
    color: Color,
    lines: &mut DebugLines,
) {
    let x_rotate = Quat::from_rotation_x(FRAC_PI_2);
    let y_rotate = Quat::from_rotation_y(FRAC_PI_2);
    let invert_semi = Quat::from_rotation_z(PI);

    let lower = [
        origin + orient.mul_vec3(Vec3::from([0.0, half_segment, -radius])),
        origin + orient.mul_vec3(Vec3::from([0.0, half_segment, radius])),
        origin + orient.mul_vec3(Vec3::from([-radius, half_segment, 0.0])),
        origin + orient.mul_vec3(Vec3::from([radius, half_segment, 0.0])),
    ];
    let upper = [
        origin + orient.mul_vec3(Vec3::from([0.0, -half_segment, -radius])),
        origin + orient.mul_vec3(Vec3::from([0.0, -half_segment, radius])),
        origin + orient.mul_vec3(Vec3::from([-radius, -half_segment, 0.0])),
        origin + orient.mul_vec3(Vec3::from([radius, -half_segment, 0.0])),
    ];
    for i in 0..4 {
        lines.line_colored(lower[i], upper[i], 0.0, color);
    }

    let lower_center = origin + orient.mul_vec3(-Vec3::Y * half_segment);
    let upper_center = origin + orient.mul_vec3(Vec3::Y * half_segment);
    add_semicircle(
        lower_center,
        orient * invert_semi * y_rotate,
        radius,
        color,
        lines,
    );
    add_semicircle(lower_center, orient * invert_semi, radius, color, lines);
    add_circle(lower_center, orient * x_rotate, radius, color, lines);

    add_semicircle(upper_center, orient * y_rotate, radius, color, lines);
    add_semicircle(upper_center, orient, radius, color, lines);
    add_circle(upper_center, orient * x_rotate, radius, color, lines);
}
pub(crate) fn add_height_field(
    origin: Vec3,
    orient: Quat,
    size: Vec2,
    heights: &[Vec<f32>],
    color: Color,
    lines: &mut DebugLines,
) {
    let y_step = size.y / (heights.len() - 1) as f32;
    let y_org = -size.y / 2.0;

    let x_length = heights[0].len() - 1;
    let x_step = size.x / x_length as f32;
    let x_org = -size.x / 2.0;

    for y_i in 0..(heights.len() - 1) {
        for x_i in 0..x_length {
            let x0 = x_org + x_i as f32 * x_step;
            let x1 = x_org + (x_i + 1) as f32 * x_step;
            let y0 = y_org + y_i as f32 * y_step;
            let y1 = y_org + (y_i + 1) as f32 * y_step;
            let p00 = origin + orient.mul_vec3(Vec3::new(x0, heights[x_i][y_i], y0));
            let p01 = origin + orient.mul_vec3(Vec3::new(x1, heights[x_i + 1][y_i], y0));
            let p10 = origin + orient.mul_vec3(Vec3::new(x0, heights[x_i][y_i + 1], y1));
            let p11 = origin + orient.mul_vec3(Vec3::new(x1, heights[x_i + 1][y_i + 1], y1));
            // NOTE: We create duplicate lines here
            lines.line_colored(p00, p01, 0.0, color);
            lines.line_colored(p00, p10, 0.0, color);
            lines.line_colored(p10, p11, 0.0, color);
            lines.line_colored(p01, p11, 0.0, color);
            lines.line_colored(p10, p01, 0.0, color);
        }
    }
}
pub(crate) fn add_convex_hull(
    origin: Vec3,
    orient: Quat,
    points: &[Vec3],
    color: Color,
    lines: &mut DebugLines,
) {
    let points3d: Vec<_> = points.into_rapier();
    let (vertex, edges) = convex_hull(&points3d);
    for edge in &edges {
        let p0 = origin + orient.mul_vec3(vertex[edge[0] as usize].into_bevy());
        let p1 = origin + orient.mul_vec3(vertex[edge[1] as usize].into_bevy());
        let p2 = origin + orient.mul_vec3(vertex[edge[2] as usize].into_bevy());
        lines.line_colored(p0, p1, 0.0, color);
        lines.line_colored(p0, p2, 0.0, color);
        lines.line_colored(p1, p2, 0.0, color);
    }
}

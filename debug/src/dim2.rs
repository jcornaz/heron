use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_prototype_lyon::prelude::*;
use bevy_render::prelude::*;
use bevy_sprite::prelude::*;
use bevy_transform::prelude::*;

use heron_core::Body;

use super::*;
pub(crate) fn create_debug_sprites(
    commands: &mut Commands,
    query: Query<'_, (Entity, &Body, &GlobalTransform), Without<HasDebug>>,
    debug_mat: Res<'_, DebugMaterial>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (entity, body, transform) in query.iter() {
        commands.set_current_entity(entity);
        commands
            .with_children(|builder| {
                builder
                    .spawn(create_sprite(
                        body,
                        material.clone(),
                        &mut meshes,
                        *transform,
                    ))
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
    mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    let material = debug_mat.handle().expect("Debug material wasn't loaded");

    for (parent_entity, body, transform) in query.iter() {
        if let Some((debug_entity, mesh)) = map.remove(&parent_entity) {
            commands.despawn(debug_entity);
            meshes.remove(mesh);
            commands.set_current_entity(parent_entity);
            commands.with_children(|builder| {
                builder
                    .spawn(create_sprite(
                        body,
                        material.clone(),
                        &mut meshes,
                        *transform,
                    ))
                    .with(IsDebug(parent_entity));
            });
        }
    }
}

pub(crate) fn delete_debug_sprite(
    commands: &mut Commands,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, Entity, (With<HasDebug>, Without<Body>)>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
) {
    for parent_entity in query.removed::<Body>() {
        if let Some((debug_entity, mesh)) = map.remove(&parent_entity) {
            meshes.remove(mesh);
            commands.despawn(debug_entity);
        }
    }
}

fn create_sprite(
    body: &Body,
    material: Handle<ColorMaterial>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    transform: GlobalTransform,
) -> SpriteBundle {
    let mut bundle = primitive(
        material,
        meshes,
        shape_type(body),
        TessellationMode::Fill(&FillOptions::default()),
        Vec3::zero(),
    );

    bundle.transform.translation.z = 1.0;
    bundle.transform.scale = transform.scale.recip();
    bundle.global_transform = transform;
    bundle.global_transform.translation.z += 1.0;
    bundle.global_transform.scale = Vec3::new(1.0, 1.0, 1.0);
    bundle
}

fn shape_type(body: &Body) -> ShapeType {
    match body {
        Body::Sphere { radius } => ShapeType::Circle(*radius),
    }
}

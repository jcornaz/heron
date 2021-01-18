#![cfg(all(feature = "debug", feature = "2d"))]

use std::ops::Deref;

use bevy_app::{AppBuilder, Plugin};
use bevy_asset::prelude::*;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use bevy_prototype_lyon::prelude::*;
use bevy_render::prelude::*;
use bevy_sprite::prelude::*;
use bevy_transform::prelude::*;
use fnv::FnvHashMap;

use heron_core::Body;

pub(crate) struct DebugPlugin(pub(crate) Color);

#[derive(Debug, Clone)]
enum DebugMaterial {
    Color(Color),
    Handle(Handle<ColorMaterial>),
}

type DebugSpriteMap = FnvHashMap<Entity, (Entity, Handle<Mesh>)>;

struct HasDebug;
struct IsDebug(Entity);
struct Indexed;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_resource(DebugMaterial::from(self.0))
            .init_resource::<DebugSpriteMap>()
            .add_stage_after(
                crate::stage::PRE_STEP,
                crate::stage::DEBUG,
                SystemStage::serial()
                    .with_system(delete_debug_sprite.system())
                    .with_system(replace_debug_sprite.system())
                    .with_system(create_debug_sprites.system())
                    .with_system(reference_debug_sprites.system())
                    .with_system(scale_debug_sprite.system()),
            )
            .add_startup_system(create_material.system());
    }
}

impl From<Color> for DebugMaterial {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl DebugMaterial {
    fn handle(&self) -> Option<&Handle<ColorMaterial>> {
        match self {
            DebugMaterial::Color(_) => None,
            DebugMaterial::Handle(handle) => Some(handle),
        }
    }
}

fn create_material(
    mut debug_mat: ResMut<DebugMaterial>,
    mut assets: ResMut<Assets<ColorMaterial>>,
) {
    if let DebugMaterial::Color(color) = debug_mat.deref() {
        *debug_mat = DebugMaterial::Handle(assets.add((*color).into()));
    }
}

fn create_debug_sprites(
    commands: &mut Commands,
    query: Query<(Entity, &Body, &GlobalTransform), Without<HasDebug>>,
    debug_mat: Res<DebugMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
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

fn reference_debug_sprites(
    commands: &mut Commands,
    mut map: ResMut<'_, DebugSpriteMap>,
    query: Query<(Entity, &IsDebug, &Handle<Mesh>), Without<Indexed>>,
) {
    for (debug_entity, IsDebug(parent_entity), mesh_handle) in query.iter() {
        map.insert(*parent_entity, (debug_entity, mesh_handle.clone()));
        commands.insert_one(debug_entity, Indexed);
    }
}

fn replace_debug_sprite(
    commands: &mut Commands,
    mut map: ResMut<'_, DebugSpriteMap>,
    query: Query<(Entity, &Body, &GlobalTransform), (With<HasDebug>, Mutated<Body>)>,
    debug_mat: Res<DebugMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
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

fn delete_debug_sprite(
    commands: &mut Commands,
    mut map: ResMut<'_, DebugSpriteMap>,
    query: Query<Entity, (With<HasDebug>, Without<Body>)>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    for parent_entity in query.removed::<Body>() {
        if let Some((debug_entity, mesh)) = map.remove(&parent_entity) {
            meshes.remove(mesh);
            commands.despawn(debug_entity);
        }
    }
}

fn scale_debug_sprite(mut query: Query<(Option<&mut Transform>, &mut GlobalTransform)>) {
    query
        .iter_mut()
        .filter(|(_, global)| {
            let scale = global.scale;
            scale.x != 1.0 || scale.y != 1.0
        })
        .for_each(|(local, mut global)| {
            if let Some(mut local) = local {
                local.scale *= global.scale.recip()
            }
            global.scale.x = 1.0;
            global.scale.y = 1.0;
            global.scale.z = 1.0;
        });
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

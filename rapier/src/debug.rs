#![cfg(feature = "debug")]

use std::ops::Deref;

use bevy_asset::{Assets, Handle};
use bevy_ecs::{Commands, Entity, Query, Res, ResMut};
use bevy_math::Vec3;
use bevy_prototype_lyon::basic_shapes::{primitive, ShapeType};
use bevy_prototype_lyon::prelude::FillOptions;
use bevy_prototype_lyon::TessellationMode;
use bevy_render::color::Color;
use bevy_render::mesh::Mesh;
use bevy_sprite::ColorMaterial;
use bevy_transform::hierarchy::BuildChildren;

use heron_core::Body;

pub(crate) enum DebugMaterial {
    Color(Color),
    Handle(Handle<ColorMaterial>),
}

impl From<Color> for DebugMaterial {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl DebugMaterial {
    pub fn init(mut debug_mat: ResMut<DebugMaterial>, mut assets: ResMut<Assets<ColorMaterial>>) {
        if let DebugMaterial::Color(color) = debug_mat.deref() {
            *debug_mat = DebugMaterial::Handle(assets.add((*color).into()));
        }
    }
}

#[cfg(feature = "2d")]
pub(crate) fn add_debug_sprites(
    commands: &mut Commands,
    query: Query<(Entity, &Body)>,
    debug_mat: Res<DebugMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let material = match debug_mat.deref() {
        DebugMaterial::Color(_) => return,
        DebugMaterial::Handle(handle) => handle,
    };

    for (entity, body) in query.iter() {
        commands.set_current_entity(entity);
        commands.with_children(|builder| {
            let mut sprite_bundle = primitive(
                material.clone(),
                &mut meshes,
                match body {
                    Body::Sphere { radius } => ShapeType::Circle(*radius),
                },
                TessellationMode::Fill(&FillOptions::default()),
                Vec3::zero(),
            );

            sprite_bundle.transform.translation.z = 1.0;
            builder.spawn(sprite_bundle);
        });
    }
}

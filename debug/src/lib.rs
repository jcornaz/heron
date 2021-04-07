#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

//! Rendering of Heron's collision shapes for debugging purposes

use bevy::prelude::*;
use fnv::FnvHashMap;

#[cfg(feature = "2d")]
mod dim2;

/// Plugin that enables rendering of collision shapes
#[derive(Debug, Copy, Clone)]
pub struct DebugPlugin(Color);

#[derive(Debug, Clone)]
enum DebugMaterial {
    Color(Color),
    Handle(Handle<ColorMaterial>),
}

type DebugEntityMap = FnvHashMap<Entity, Entity>;

#[allow(unused)]
struct HasDebug;

#[allow(unused)]
struct IsDebug(Entity);

#[allow(unused)]
struct Indexed;

impl From<Color> for DebugPlugin {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl Default for DebugPlugin {
    fn default() -> Self {
        let mut color = bevy::render::color::Color::BLUE;
        color.set_a(0.4);
        Self(color)
    }
}

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(feature = "2d")]
        app.add_plugin(bevy_prototype_lyon::plugin::ShapePlugin);

        app.insert_resource(DebugMaterial::from(self.0))
            .init_resource::<DebugEntityMap>()
            .stage(heron_core::stage::ROOT, |schedule: &mut Schedule| {
                schedule.add_stage_after(heron_core::stage::UPDATE, "heron-debug", debug_stage())
            })
            .add_startup_system(create_material.system());
    }
}

fn debug_stage() -> SystemStage {
    let mut stage = SystemStage::serial();

    #[cfg(feature = "2d")]
    {
        stage
            .add_system(dim2::delete_debug_sprite.system())
            .add_system(dim2::replace_debug_sprite.system())
            .add_system(dim2::create_debug_sprites.system());
    }

    stage
        .add_system(track_debug_entities.system())
        .add_system(scale_debug_entities.system());

    stage
}

impl From<Color> for DebugMaterial {
    fn from(color: Color) -> Self {
        Self::Color(color)
    }
}

impl DebugMaterial {
    #[allow(unused)]
    fn handle(&self) -> Option<&Handle<ColorMaterial>> {
        match self {
            DebugMaterial::Color(_) => None,
            DebugMaterial::Handle(handle) => Some(handle),
        }
    }
}

fn create_material(
    mut debug_mat: ResMut<'_, DebugMaterial>,
    mut assets: ResMut<'_, Assets<ColorMaterial>>,
) {
    if let DebugMaterial::Color(color) = &*debug_mat {
        *debug_mat = DebugMaterial::Handle(assets.add((*color).into()));
    }
}

fn track_debug_entities(
    mut commands: Commands,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, (Entity, &IsDebug), Without<Indexed>>,
) {
    for (debug_entity, IsDebug(parent_entity)) in query.iter() {
        map.insert(*parent_entity, debug_entity);
        commands.insert(debug_entity, Indexed);
    }
}

fn scale_debug_entities(mut query: Query<'_, (Option<&mut Transform>, &mut GlobalTransform)>) {
    query
        .iter_mut()
        .filter(|(_, global)| {
            let scale = global.scale;
            !is_near(scale.x, 1.0) || !is_near(scale.y, 1.0)
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

#[inline]
fn is_near(v1: f32, v2: f32) -> bool {
    (v2 - v1).abs() <= f32::EPSILON
}

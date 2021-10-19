#![deny(future_incompatible, nonstandard_style)]
#![warn(missing_docs, rust_2018_idioms, clippy::pedantic)]
#![allow(clippy::needless_pass_by_value)]

//! Rendering of Heron's collision shapes for debugging purposes

use bevy::prelude::*;
use fnv::FnvHashMap;

#[cfg(all(feature = "2d", not(feature = "3d")))]
mod dim2;

/// Plugin that enables rendering of collision shapes
#[derive(Debug, Copy, Clone, Default)]
pub struct DebugPlugin(DebugColor);

#[derive(Debug, Copy, Clone)]
struct DebugColor {
    sensor: Color,
    static_body: Color,
    dynamic_body: Color,
    kinematic_body: Color,
}

type DebugEntityMap = FnvHashMap<Entity, Entity>;

#[allow(unused)]
struct HasDebug;

#[allow(unused)]
struct IsDebug(Entity);

#[allow(unused)]
struct Indexed;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut AppBuilder) {
        #[cfg(all(feature = "2d", not(feature = "3d")))]
        app.add_plugin(bevy_prototype_lyon::plugin::ShapePlugin)
            .add_system_set_to_stage(CoreStage::PostUpdate, dim2::systems());

        app.insert_resource(self.0)
            .init_resource::<DebugEntityMap>()
            .add_system_to_stage(CoreStage::Last, track_debug_entities.system())
            .add_system_to_stage(CoreStage::Last, scale_debug_entities.system());
    }
}

impl Default for DebugColor {
    fn default() -> Self {
        Self {
            sensor: Color::rgba(0.0, 0.63, 0.0, 0.4),
            static_body: Color::rgba(0.64, 0.0, 0.16, 0.4),
            dynamic_body: Color::rgba(0.0, 0.18, 0.54, 0.4),
            kinematic_body: Color::rgba(0.21, 0.07, 0.7, 0.4),
        }
    }
}

fn track_debug_entities(
    mut commands: Commands<'_>,
    mut map: ResMut<'_, DebugEntityMap>,
    query: Query<'_, (Entity, &IsDebug), Without<Indexed>>,
) {
    for (debug_entity, IsDebug(parent_entity)) in query.iter() {
        map.insert(*parent_entity, debug_entity);
        commands.entity(debug_entity).insert(Indexed);
    }
}

fn scale_debug_entities(
    mut query: Query<'_, (Option<&mut Transform>, &mut GlobalTransform), With<IsDebug>>,
) {
    query
        .iter_mut()
        .filter(|(_, global)| {
            let scale = global.scale;
            !is_near(scale.x, 1.0) || !is_near(scale.y, 1.0)
        })
        .for_each(|(local, mut global)| {
            if let Some(mut local) = local {
                local.scale *= global.scale.recip();
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

#![cfg(any(dim2, dim3))]

use bevy::math::prelude::*;
use bevy::reflect::TypeRegistryArc;
use bevy::{app::prelude::*, prelude::Schedule};
use bevy::{core::CorePlugin, prelude::StageLabel};

use heron_core::Gravity;
use heron_core::PhysicsTime;
use heron_rapier::{RapierPlugin, StagedRapierPlugin};
use utils::*;

mod utils;

#[test]
fn can_define_gravity_before_plugin() {
    let app = {
        let mut builder = App::new();

        builder
            .insert_resource(Gravity::from(Vec3::Y))
            .init_resource::<TypeRegistryArc>()
            .add_plugin(CorePlugin)
            .add_plugin(RapierPlugin::default());

        builder
    };

    assert_eq!(
        Vec3::Y,
        app.world.get_resource::<Gravity>().unwrap().vector()
    );
}

#[test]
fn can_define_time_scale_before_plugin() {
    let app = {
        let mut builder = App::new();

        builder
            .insert_resource(PhysicsTime::new(0.5))
            .init_resource::<TypeRegistryArc>()
            .add_plugin(CorePlugin)
            .add_plugin(RapierPlugin::default());

        builder
    };

    assert_eq!(
        0.5,
        app.world.get_resource::<PhysicsTime>().unwrap().scale()
    );
}

#[test]
fn rapier_world_is_registered() {
    let mut app = App::new();
    app.init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin::default());

    assert!(app.world.contains_resource::<RigidBodySet>());
    assert!(app.world.contains_resource::<ColliderSet>());
    assert!(app.world.contains_resource::<JointSet>());
    assert!(app.world.contains_resource::<IntegrationParameters>());
}

#[test]
fn use_passed_stage_labels() {
    let mut app = App::new();
    let physics_schedule = "Physics-stage";
    let post_physics_stage = CoreStage::PostUpdate;
    let step_physics_stage = CoreStage::First;
    app.init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_stage_before(
            bevy::prelude::CoreStage::PostUpdate,
            physics_schedule,
            Schedule::default(),
        )
        .add_plugin(StagedRapierPlugin {
            physics_schedule,
            post_physics_stage,
            step_physics_stage,
        });

    let schedule = app
        .schedule
        .get_stage::<Schedule>(&physics_schedule)
        .expect("Schedule should exist");

    let stage_labels: Vec<&dyn StageLabel> =
        schedule.iter_stages().map(|(label, _)| label).collect();

    assert_eq!(stage_labels.len(), 4);
}

#![cfg(any(dim2, dim3))]

use bevy::app::prelude::*;
use bevy::core::CorePlugin;
use bevy::math::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::Gravity;
use heron_core::PhysicsTime;
use heron_rapier::RapierPlugin;
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

    assert_eq!(Vec3::Y, app.world.resource::<Gravity>().vector());
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

    assert_eq!(0.5, app.world.resource::<PhysicsTime>().scale());
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

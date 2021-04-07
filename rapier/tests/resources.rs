#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::app::prelude::*;
use bevy::core::CorePlugin;
use bevy::math::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::Gravity;
use heron_rapier::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use heron_rapier::rapier::geometry::ColliderSet;
use heron_rapier::RapierPlugin;

#[test]
fn can_define_gravity_before_plugin() {
    let mut app = App::build();
    app.insert_resource(Gravity::from(Vec3::unit_y()))
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin::default());

    assert_eq!(
        Vec3::unit_y(),
        app.resources().get::<Gravity>().unwrap().vector()
    );
}

#[test]
fn rapier_world_is_registered() {
    let mut app = App::build();
    app.init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin::default());

    assert!(app.resources().contains::<RigidBodySet>());
    assert!(app.resources().contains::<ColliderSet>());
    assert!(app.resources().contains::<JointSet>());
    assert!(app.resources().contains::<IntegrationParameters>());
}

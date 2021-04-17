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
    let app = {
        let mut builder = App::build();

        builder
            .insert_resource(Gravity::from(Vec3::Y))
            .init_resource::<TypeRegistryArc>()
            .add_plugin(CorePlugin)
            .add_plugin(RapierPlugin::default());

        builder.app
    };

    assert_eq!(
        Vec3::Y,
        app.world.get_resource::<Gravity>().unwrap().vector()
    );
}

#[test]
fn rapier_world_is_registered() {
    let mut app = App::build();
    app.init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin::default());

    assert!(app.world().contains_resource::<RigidBodySet>());
    assert!(app.world().contains_resource::<ColliderSet>());
    assert!(app.world().contains_resource::<JointSet>());
    assert!(app.world().contains_resource::<IntegrationParameters>());
}

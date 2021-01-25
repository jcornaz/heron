#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::Body;
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters: IntegrationParameters::default(),
        });
    builder.app
}

#[test]
#[ignore]
fn collision_events_are_fired() {
    let mut app = test_app();

    let entity1 = app.world.spawn((
        Transform::default(),
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
    ));

    let entity2 = app.world.spawn((
        Transform::from_translation(Vec3::unit_x() * 2.0),
        GlobalTransform::default(),
        Body::Sphere { radius: 10.0 },
    ));

    app.update();

    let mut transform = app.world.get_mut::<Transform>(entity2).unwrap();
    transform.translation.x = 5.0;

    app.update();

    todo!("Read events")
}

#[test]
#[ignore]
fn other_components_can_be_queried_from_a_collision() {
    todo!()
}

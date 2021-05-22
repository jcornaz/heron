#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::app::{Events, ManualEventReader};
use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{CollisionEvent, CollisionShape, RigidBody, Velocity};
use heron_rapier::rapier::dynamics::IntegrationParameters;
use heron_rapier::RapierPlugin;

fn test_app() -> App {
    let mut builder = App::build();
    let mut parameters = IntegrationParameters::default();
    parameters.dt = 1.0;

    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin {
            step_per_second: None,
            parameters,
        })
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::transform::transform_propagate_system::transform_propagate_system.system(),
        );
    builder.app
}

#[test]
fn collision_events_are_fired() {
    let mut app = test_app();

    let entity1 = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            RigidBody::Sensor,
        ))
        .id();

    let entity2 = app
        .world
        .spawn()
        .insert_bundle((
            Transform::from_translation(Vec3::X * -30.0),
            GlobalTransform::default(),
            RigidBody::Dynamic,
            CollisionShape::Sphere { radius: 10.0 },
            Velocity::from_linear(Vec3::X * 30.0),
        ))
        .id();

    let mut event_reader = app
        .world
        .get_resource::<Events<CollisionEvent>>()
        .unwrap()
        .get_reader();

    let mut events = vec![];

    app.update();
    events.append(&mut collect_events(&app, &mut event_reader));

    app.update();
    events.append(&mut collect_events(&app, &mut event_reader));

    assert_eq!(
        events,
        vec![
            CollisionEvent::Started(entity1, entity2),
            CollisionEvent::Stopped(entity1, entity2)
        ]
    );
}

fn collect_events(
    app: &App,
    reader: &mut ManualEventReader<CollisionEvent>,
) -> Vec<CollisionEvent> {
    let events = app.world.get_resource::<Events<CollisionEvent>>().unwrap();
    reader.iter(&events).copied().collect()
}

#![cfg(all(
    any(feature = "2d", feature = "3d"),
    not(all(feature = "2d", feature = "3d")),
))]

use bevy::app::{Events, ManualEventReader};
use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron_core::{Body, CollisionEvent};
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
            Body::Sphere { radius: 10.0 },
        ))
        .id();

    let entity2 = app
        .world
        .spawn()
        .insert_bundle((
            Transform::from_translation(Vec3::X * 30.0),
            GlobalTransform::default(),
            Body::Sphere { radius: 10.0 },
        ))
        .id();

    app.update();

    app.world
        .get_mut::<Transform>(entity2)
        .unwrap()
        .translation
        .x = 10.0;
    app.update();

    let mut event_reader = app
        .world
        .get_resource::<Events<CollisionEvent>>()
        .unwrap()
        .get_reader();

    let events = collect_events(&app, &mut event_reader);

    assert_eq!(events, vec![CollisionEvent::Started(entity1, entity2)]);

    app.world
        .get_mut::<Transform>(entity2)
        .unwrap()
        .translation
        .x = 30.0;
    app.update();

    let events = collect_events(&app, &mut event_reader);

    assert_eq!(events, vec![CollisionEvent::Stopped(entity1, entity2)])
}

fn collect_events(
    app: &App,
    reader: &mut ManualEventReader<CollisionEvent>,
) -> Vec<CollisionEvent> {
    let events = app.world.get_resource::<Events<CollisionEvent>>().unwrap();
    reader.iter(&events).cloned().collect()
}

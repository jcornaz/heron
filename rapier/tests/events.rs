#![cfg(any(dim2, dim3))]

use std::time::Duration;

use bevy::app::{Events, ManualEventReader};
use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;
use rstest::*;

use heron_core::{CollisionEvent, CollisionShape, PhysicsSteps, RigidBody, Velocity};
use heron_rapier::RapierPlugin;
use utils::*;

mod utils;

fn test_app() -> App {
    let mut builder = App::new();
    let mut parameters = IntegrationParameters::default();
    parameters.dt = 1.0;

    builder
        .init_resource::<TypeRegistryArc>()
        .insert_resource(PhysicsSteps::every_frame(Duration::from_secs(1)))
        .add_plugin(CorePlugin)
        .add_plugin(RapierPlugin)
        .add_system_to_stage(
            bevy::app::CoreStage::PostUpdate,
            bevy::transform::transform_propagate_system::transform_propagate_system,
        );
    builder
}

#[rstest]
#[case(RigidBody::Sensor, RigidBody::Dynamic)]
#[case(RigidBody::Sensor, RigidBody::Sensor)]
#[case(RigidBody::Sensor, RigidBody::KinematicPositionBased)]
#[case(RigidBody::Sensor, RigidBody::KinematicVelocityBased)]
#[case(RigidBody::Dynamic, RigidBody::Dynamic)]
fn collision_events_are_fired(#[case] type1: RigidBody, #[case] type2: RigidBody) {
    let mut app = test_app();

    let entity1 = app
        .world
        .spawn()
        .insert_bundle((
            Transform::default(),
            GlobalTransform::default(),
            CollisionShape::Sphere { radius: 10.0 },
            type1,
        ))
        .id();

    let entity2 = app
        .world
        .spawn()
        .insert_bundle((
            Transform::from_translation(Vec3::X * -30.0),
            GlobalTransform::default(),
            type2,
            CollisionShape::Sphere { radius: 10.0 },
        ))
        .id();

    if type2.can_have_velocity() {
        app.world
            .entity_mut(entity2)
            .insert(Velocity::from_linear(Vec3::X * 30.0));
    } else {
        app.world
            .get_mut::<Transform>(entity2)
            .unwrap()
            .translation
            .x += 30.0;
    }

    let mut event_reader = app
        .world
        .get_resource::<Events<CollisionEvent>>()
        .unwrap()
        .get_reader();

    let mut events = vec![];

    app.update();
    events.append(&mut collect_events(&app, &mut event_reader));

    if !type2.can_have_velocity() {
        app.world
            .get_mut::<Transform>(entity2)
            .unwrap()
            .translation
            .x += 30.0;
    }

    app.update();
    events.append(&mut collect_events(&app, &mut event_reader));

    assert_eq!(events.len(), 2);
    assert!(matches!(&events[0], CollisionEvent::Started(_, _)));
    assert!(matches!(&events[1], CollisionEvent::Stopped(_, _)));
    assert_eq!(events[0].collision_shape_entities(), (entity1, entity2));
    assert_eq!(events[1].collision_shape_entities(), (entity1, entity2));

    match (type1, type2) {
        (RigidBody::Sensor, RigidBody::Dynamic) => {
            assert!(
                matches!(&events[0], CollisionEvent::Started(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
            assert!(
                matches!(&events[1], CollisionEvent::Stopped(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
        }
        (RigidBody::Sensor, RigidBody::Sensor) => {
            assert!(
                matches!(&events[0], CollisionEvent::Started(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
            assert!(
                matches!(&events[1], CollisionEvent::Stopped(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
        }
        (RigidBody::Sensor, RigidBody::KinematicPositionBased) => {
            assert!(
                matches!(&events[0], CollisionEvent::Started(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
            assert!(
                matches!(&events[1], CollisionEvent::Stopped(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
        }
        (RigidBody::Sensor, RigidBody::KinematicVelocityBased) => {
            assert!(
                matches!(&events[0], CollisionEvent::Started(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
            assert!(
                matches!(&events[1], CollisionEvent::Stopped(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
        }
        (RigidBody::Dynamic, RigidBody::Dynamic) => {
            assert!(
                matches!(&events[0], CollisionEvent::Started(c1, c2) if c1.normals().len() == 1 && c2.normals().len() == 1)
            );
            assert!(
                matches!(&events[1], CollisionEvent::Stopped(c1, c2) if c1.normals().is_empty() && c2.normals().is_empty())
            );
        }
        _ => unimplemented!(),
    }
}

fn collect_events(
    app: &App,
    reader: &mut ManualEventReader<CollisionEvent>,
) -> Vec<CollisionEvent> {
    let events = app.world.get_resource::<Events<CollisionEvent>>().unwrap();
    reader.iter(events).cloned().collect()
}

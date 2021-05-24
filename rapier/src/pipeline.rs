use bevy::app::Events;
use bevy::ecs::prelude::*;
use bevy::math::Vec3;
use crossbeam::channel::Receiver;

use heron_core::{CollisionData, CollisionEvent, Gravity, PhysicsTime};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{CCDSolver, IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{
    BroadPhase, ColliderHandle, ColliderSet, ContactEvent, IntersectionEvent, NarrowPhase,
};
use crate::rapier::pipeline::{ChannelEventCollector, PhysicsPipeline};

#[derive(Copy, Clone)]
pub(crate) struct PhysicsStepPerSecond(pub(crate) f32);

pub(crate) fn update_integration_parameters(
    steps_per_second: Option<Res<'_, PhysicsStepPerSecond>>,
    physics_time: Res<'_, PhysicsTime>,
    mut integration_parameters: ResMut<'_, IntegrationParameters>,
) {
    if let Some(steps_per_second) = steps_per_second {
        if steps_per_second.is_changed() {
            integration_parameters.dt = physics_time.scale() / steps_per_second.0;
        }
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn step(
    mut pipeline: ResMut<'_, PhysicsPipeline>,
    gravity: Res<'_, Gravity>,
    integration_parameters: Res<'_, IntegrationParameters>,
    mut broad_phase: ResMut<'_, BroadPhase>,
    mut narrow_phase: ResMut<'_, NarrowPhase>,
    mut bodies: ResMut<'_, RigidBodySet>,
    mut colliders: ResMut<'_, ColliderSet>,
    mut joints: ResMut<'_, JointSet>,
    mut ccd_solver: ResMut<'_, CCDSolver>,
    event_manager: Local<'_, EventManager>,
    mut events: ResMut<'_, Events<CollisionEvent>>,
) {
    let gravity = Vec3::from(*gravity).into_rapier();
    pipeline.step(
        &gravity,
        &integration_parameters,
        &mut broad_phase,
        &mut narrow_phase,
        &mut bodies,
        &mut colliders,
        &mut joints,
        &mut ccd_solver,
        &(),
        &event_manager.handler,
    );

    event_manager.fire_events(&bodies, &colliders, &mut events);
}

pub(crate) struct EventManager {
    contacts: Receiver<ContactEvent>,
    intersections: Receiver<IntersectionEvent>,
    handler: ChannelEventCollector,
}

impl Default for EventManager {
    fn default() -> Self {
        let (contact_send, contacts) = crossbeam::channel::unbounded();
        let (proximity_send, proximities) = crossbeam::channel::unbounded();
        let handler = ChannelEventCollector::new(proximity_send, contact_send);
        Self {
            contacts,
            handler,
            intersections: proximities,
        }
    }
}

impl EventManager {
    fn fire_events(
        &self,
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        events: &mut Events<CollisionEvent>,
    ) {
        while let Ok(event) = self.contacts.try_recv() {
            match event {
                ContactEvent::Started(h1, h2) => {
                    if let Some((d1, d2)) = Self::data(bodies, colliders, h1, h2) {
                        events.send(CollisionEvent::Started(d1, d2));
                    }
                }
                ContactEvent::Stopped(h1, h2) => {
                    if let Some((d1, d2)) = Self::data(bodies, colliders, h1, h2) {
                        events.send(CollisionEvent::Stopped(d1, d2));
                    }
                }
            }
        }

        while let Ok(IntersectionEvent {
            collider1,
            collider2,
            intersecting,
        }) = self.intersections.try_recv()
        {
            if let Some((e1, e2)) = Self::data(bodies, colliders, collider1, collider2) {
                if intersecting {
                    events.send(CollisionEvent::Started(e1, e2));
                } else {
                    events.send(CollisionEvent::Stopped(e1, e2));
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn data(
        bodies: &RigidBodySet,
        colliders: &ColliderSet,
        h1: ColliderHandle,
        h2: ColliderHandle,
    ) -> Option<(CollisionData, CollisionData)> {
        if let (Some(collider1), Some(collider2)) = (colliders.get(h1), colliders.get(h2)) {
            if let (Some(rb1), Some(rb2)) = (
                bodies.get(collider1.parent()),
                bodies.get(collider2.parent()),
            ) {
                let d1 = CollisionData::new(
                    Entity::from_bits(rb1.user_data as u64),
                    Entity::from_bits(collider1.user_data as u64),
                    collider1.collision_groups().into_bevy(),
                );
                let d2 = CollisionData::new(
                    Entity::from_bits(rb2.user_data as u64),
                    Entity::from_bits(collider2.user_data as u64),
                    collider2.collision_groups().into_bevy(),
                );
                Some(
                    if Entity::from_bits(rb1.user_data as u64)
                        < Entity::from_bits(rb2.user_data as u64)
                    {
                        (d1, d2)
                    } else {
                        (d1, d2)
                    },
                )
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use heron_core::CollisionLayers;

    use crate::pipeline::EventManager;
    use crate::rapier::dynamics::RigidBodyBuilder;
    use crate::rapier::geometry::{ColliderBuilder, ColliderHandle};
    use crate::rapier::pipeline::EventHandler;

    use super::*;

    struct TestContext {
        bodies: RigidBodySet,
        colliders: ColliderSet,
        rb_entity_1: Entity,
        rb_entity_2: Entity,
        collider_entity_1: Entity,
        collider_entity_2: Entity,
        layers_1: CollisionLayers,
        layers_2: CollisionLayers,
        handle1: ColliderHandle,
        handle2: ColliderHandle,
    }

    impl Default for TestContext {
        fn default() -> Self {
            let mut bodies = RigidBodySet::new();
            let mut colliders = ColliderSet::new();

            let rb_entity_1 = Entity::new(0);
            let rb_entity_2 = Entity::new(1);
            let collider_entity_1 = Entity::new(2);
            let collider_entity_2 = Entity::new(3);
            let layers_1 = CollisionLayers::from_bits(1, 2);
            let layers_2 = CollisionLayers::from_bits(3, 4);
            let body1 = bodies.insert(
                RigidBodyBuilder::new_dynamic()
                    .user_data(rb_entity_1.to_bits().into())
                    .build(),
            );
            let body2 = bodies.insert(
                RigidBodyBuilder::new_dynamic()
                    .user_data(rb_entity_2.to_bits().into())
                    .build(),
            );
            let handle1 = colliders.insert(
                ColliderBuilder::ball(1.0)
                    .user_data(collider_entity_1.to_bits().into())
                    .collision_groups(layers_1.into_rapier())
                    .build(),
                body1,
                &mut bodies,
            );
            let handle2 = colliders.insert(
                ColliderBuilder::ball(1.0)
                    .user_data(collider_entity_2.to_bits().into())
                    .collision_groups(layers_2.into_rapier())
                    .build(),
                body2,
                &mut bodies,
            );

            Self {
                bodies,
                colliders,
                rb_entity_1,
                rb_entity_2,
                collider_entity_1,
                collider_entity_2,
                layers_1,
                layers_2,
                handle1,
                handle2,
            }
        }
    }

    #[test]
    fn contact_started_fires_collision_started() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_contact_event(ContactEvent::Started(context.handle1, context.handle2));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).copied().collect();

        assert_eq!(events.len(), 1);
        let event = events[0];
        assert!(matches!(event, CollisionEvent::Started(_, _)));
        assert_eq!(
            event.collision_shape_entities(),
            (context.collider_entity_1, context.collider_entity_2)
        );
    }

    #[test]
    fn contact_stopped_fires_collision_stopped() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_contact_event(ContactEvent::Stopped(context.handle1, context.handle2));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).copied().collect();

        assert_eq!(events.len(), 1);
        let event = events[0];
        assert!(matches!(event, CollisionEvent::Stopped(_, _)));
        assert_eq!(
            event.collision_shape_entities(),
            (context.collider_entity_1, context.collider_entity_2)
        );
    }

    #[test]
    fn intersection_true_fires_collision_started() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_intersection_event(IntersectionEvent::new(
                context.handle1,
                context.handle2,
                true,
            ));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).copied().collect();

        assert_eq!(events.len(), 1);
        let event = events[0];
        assert!(matches!(event, CollisionEvent::Started(_, _)));
        assert_eq!(
            event.collision_shape_entities(),
            (context.collider_entity_1, context.collider_entity_2)
        );
    }

    #[test]
    fn intersection_false_fires_collision_stopped() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_intersection_event(IntersectionEvent::new(
                context.handle1,
                context.handle2,
                false,
            ));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).copied().collect();

        assert_eq!(events.len(), 1);
        let event = events[0];
        assert!(matches!(event, CollisionEvent::Stopped(_, _)));
        assert_eq!(
            event.collision_shape_entities(),
            (context.collider_entity_1, context.collider_entity_2)
        );
    }

    #[test]
    fn contains_rigid_body_entities() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_contact_event(ContactEvent::Started(context.handle1, context.handle2));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        assert_eq!(
            events
                .get_reader()
                .iter(&events)
                .next()
                .unwrap()
                .rigid_body_entities(),
            (context.rb_entity_1, context.rb_entity_2)
        );
    }

    #[test]
    fn contains_collision_layers() {
        let manager = EventManager::default();
        let context = TestContext::default();

        manager
            .handler
            .handle_contact_event(ContactEvent::Started(context.handle1, context.handle2));

        let mut events = Events::<CollisionEvent>::default();
        manager.fire_events(&context.bodies, &context.colliders, &mut events);
        assert_eq!(
            events
                .get_reader()
                .iter(&events)
                .next()
                .unwrap()
                .collision_layers(),
            (context.layers_1, context.layers_2)
        );
    }
}

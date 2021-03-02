use bevy_app::Events;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use crossbeam::channel::Receiver;

use heron_core::{CollisionEvent, Gravity};

use crate::convert::IntoRapier;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{
    BroadPhase, ColliderHandle, ColliderSet, ContactEvent, IntersectionEvent, NarrowPhase,
};
use crate::rapier::pipeline::{ChannelEventCollector, PhysicsPipeline};

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
        &(),
        &event_manager.handler,
    );

    event_manager.fire_events(&colliders, &mut events);
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
    fn fire_events(&self, colliders: &ColliderSet, events: &mut Events<CollisionEvent>) {
        while let Ok(event) = self.contacts.try_recv() {
            match event {
                ContactEvent::Started(h1, h2) => {
                    if let Some((e1, e2)) = Self::entity_pair(colliders, h1, h2) {
                        events.send(CollisionEvent::Started(e1, e2));
                    }
                }
                ContactEvent::Stopped(h1, h2) => {
                    if let Some((e1, e2)) = Self::entity_pair(colliders, h1, h2) {
                        events.send(CollisionEvent::Stopped(e1, e2));
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
            if let Some((e1, e2)) = Self::entity_pair(colliders, collider1, collider2) {
                if intersecting {
                    events.send(CollisionEvent::Started(e1, e2));
                } else {
                    events.send(CollisionEvent::Stopped(e1, e2));
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn entity_pair(
        colliders: &ColliderSet,
        h1: ColliderHandle,
        h2: ColliderHandle,
    ) -> Option<(Entity, Entity)> {
        if let (Some(b1), Some(b2)) = (colliders.get(h1), colliders.get(h2)) {
            let e1 = Entity::from_bits(b1.user_data as u64);
            let e2 = Entity::from_bits(b2.user_data as u64);
            Some(if e1 < e2 { (e1, e2) } else { (e2, e2) })
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::pipeline::EventManager;
    use crate::rapier::dynamics::RigidBodyBuilder;
    use crate::rapier::geometry::{ColliderBuilder, ColliderHandle};
    use crate::rapier::pipeline::EventHandler;

    use super::*;

    struct TestContext {
        colliders: ColliderSet,
        entity1: Entity,
        entity2: Entity,
        handle1: ColliderHandle,
        handle2: ColliderHandle,
    }

    impl Default for TestContext {
        fn default() -> Self {
            let mut bodies = RigidBodySet::new();
            let mut colliders = ColliderSet::new();

            let entity1 = Entity::new(0);
            let entity2 = Entity::new(1);
            let body1 = bodies.insert(RigidBodyBuilder::new_dynamic().build());
            let body2 = bodies.insert(RigidBodyBuilder::new_dynamic().build());
            let handle1 = colliders.insert(
                ColliderBuilder::ball(1.0)
                    .user_data(entity1.to_bits().into())
                    .build(),
                body1,
                &mut bodies,
            );
            let handle2 = colliders.insert(
                ColliderBuilder::ball(1.0)
                    .user_data(entity2.to_bits().into())
                    .build(),
                body2,
                &mut bodies,
            );

            Self {
                colliders,
                entity1,
                entity2,
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
        manager.fire_events(&context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).cloned().collect();

        assert_eq!(
            events,
            vec![CollisionEvent::Started(context.entity1, context.entity2),]
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
        manager.fire_events(&context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).cloned().collect();

        assert_eq!(
            events,
            vec![CollisionEvent::Stopped(context.entity1, context.entity2),]
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
        manager.fire_events(&context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).cloned().collect();

        assert_eq!(
            events,
            vec![CollisionEvent::Started(context.entity1, context.entity2),]
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
        manager.fire_events(&context.colliders, &mut events);
        let events: Vec<CollisionEvent> = events.get_reader().iter(&events).cloned().collect();

        assert_eq!(
            events,
            vec![CollisionEvent::Stopped(context.entity1, context.entity2)]
        );
    }
}

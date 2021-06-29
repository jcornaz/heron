use bevy::app::Events;
use bevy::ecs::prelude::*;
use bevy::log::prelude::*;
use bevy::math::Vec3;
use crossbeam::channel::{Receiver, Sender};

use heron_core::{CollisionData, CollisionEvent, Gravity, PhysicsSteps, PhysicsTime};

use crate::convert::{IntoBevy, IntoRapier};
use crate::rapier::dynamics::{
    CCDSolver, IntegrationParameters, IslandManager, JointSet, RigidBodySet,
};
use crate::rapier::geometry::{
    BroadPhase, ColliderHandle, ColliderSet, ContactEvent, IntersectionEvent, NarrowPhase,
};
use crate::rapier::pipeline::{EventHandler, PhysicsPipeline};

pub(crate) fn update_integration_parameters(
    physics_steps: Res<'_, PhysicsSteps>,
    physics_time: Res<'_, PhysicsTime>,
    mut integration_parameters: ResMut<'_, IntegrationParameters>,
) {
    if physics_steps.is_changed() || physics_time.is_changed() {
        integration_parameters.dt = physics_steps.duration().as_secs_f32() * physics_time.scale();
    }
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn step(
    mut pipeline: ResMut<'_, PhysicsPipeline>,
    gravity: Res<'_, Gravity>,
    integration_parameters: Res<'_, IntegrationParameters>,
    mut islands: ResMut<'_, IslandManager>,
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
        &mut islands,
        &mut broad_phase,
        &mut narrow_phase,
        &mut bodies,
        &mut colliders,
        &mut joints,
        &mut ccd_solver,
        &(),
        &*event_manager,
    );

    event_manager.fire_events(&bodies, &colliders, &mut events);
}

pub(crate) struct EventManager {
    contact_recv: Receiver<ContactEvent>,
    intersection_recv: Receiver<IntersectionEvent>,
    contact_send: Sender<ContactEvent>,
    intersection_send: Sender<IntersectionEvent>,
}

impl EventHandler for EventManager {
    fn handle_intersection_event(&self, event: IntersectionEvent) {
        if self.intersection_send.send(event).is_err() {
            error!("Failed to forward intersection event!")
        }
    }

    fn handle_contact_event(&self, event: ContactEvent, _: &crate::rapier::prelude::ContactPair) {
        if self.contact_send.send(event).is_err() {
            error!("Failed to forward contact event!")
        }
    }
}

impl Default for EventManager {
    fn default() -> Self {
        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        Self {
            contact_recv,
            intersection_recv,
            contact_send,
            intersection_send,
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
        while let Ok(event) = self.contact_recv.try_recv() {
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
        }) = self.intersection_recv.try_recv()
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
                collider1.parent().and_then(|parent| bodies.get(parent)),
                collider2.parent().and_then(|parent| bodies.get(parent)),
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
                        (d2, d1)
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
            let handle1 = colliders.insert_with_parent(
                ColliderBuilder::ball(1.0)
                    .user_data(collider_entity_1.to_bits().into())
                    .collision_groups(layers_1.into_rapier())
                    .build(),
                body1,
                &mut bodies,
            );
            let handle2 = colliders.insert_with_parent(
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
            .contact_send
            .send(ContactEvent::Started(context.handle1, context.handle2))
            .unwrap();

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
            .contact_send
            .send(ContactEvent::Stopped(context.handle1, context.handle2))
            .unwrap();

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
            .intersection_send
            .send(IntersectionEvent::new(
                context.handle1,
                context.handle2,
                true,
            ))
            .unwrap();

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
            .intersection_send
            .send(IntersectionEvent::new(
                context.handle1,
                context.handle2,
                false,
            ))
            .unwrap();

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
            .contact_send
            .send(ContactEvent::Started(context.handle1, context.handle2))
            .unwrap();

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
            .contact_send
            .send(ContactEvent::Started(context.handle1, context.handle2))
            .unwrap();

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

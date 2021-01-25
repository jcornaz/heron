use bevy_app::Events;
use bevy_ecs::prelude::*;
use bevy_math::Vec3;
use crossbeam::channel::Receiver;

use heron_core::{CollisionEvent, Gravity};

use crate::convert::IntoRapier;
use crate::rapier::data::arena::Index;
use crate::rapier::dynamics::{IntegrationParameters, JointSet, RigidBodySet};
use crate::rapier::geometry::{BroadPhase, ColliderSet, ContactEvent, NarrowPhase};
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
        None,
        None,
        &event_manager.handler,
    );

    event_manager.fire_events(&bodies, &mut events);
}

pub(crate) struct EventManager {
    contacts: Receiver<ContactEvent>,
    handler: ChannelEventCollector,
}

impl Default for EventManager {
    fn default() -> Self {
        let (contact_send, contacts) = crossbeam::channel::unbounded();
        let (proximity_send, _) = crossbeam::channel::unbounded();
        let handler = ChannelEventCollector::new(proximity_send, contact_send);
        Self { contacts, handler }
    }
}

impl EventManager {
    fn fire_events(&self, bodies: &RigidBodySet, events: &mut Events<CollisionEvent>) {
        while let Ok(event) = self.contacts.try_recv() {
            match event {
                ContactEvent::Started(h1, h2) => {
                    println!("started");
                    if let Some((e1, e2)) = Self::entity_pair(bodies, h1, h2) {
                        events.send(CollisionEvent::Started(e1, e2));
                    }
                }
                ContactEvent::Stopped(h1, h2) => {
                    println!("stopped");
                    if let Some((e1, e2)) = Self::entity_pair(bodies, h1, h2) {
                        events.send(CollisionEvent::Stopped(e1, e2));
                    }
                }
            }
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    fn entity_pair(bodies: &RigidBodySet, h1: Index, h2: Index) -> Option<(Entity, Entity)> {
        if let (Some(b1), Some(b2)) = (bodies.get(h1), bodies.get(h2)) {
            let e1 = Entity::from_bits(b1.user_data as u64);
            let e2 = Entity::from_bits(b2.user_data as u64);
            Some(if e1 < e2 { (e1, e2) } else { (e2, e2) })
        } else {
            None
        }
    }
}

use bevy::ecs::entity::Entity;

/// An event fired when the collision state between two entities changed
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn detect_collisions(mut events: EventReader<CollisionEvent>) {
///     for event in events.iter() {
///         match event {
///             CollisionEvent::Started(entity1, entity2) => println!("Entity {:?} and {:?} started to collide", entity1, entity2),
///             CollisionEvent::Stopped(entity1, entity2) => println!("Entity {:?} and {:?} stopped to collide", entity1, entity2),
///         }
///     }
/// }
/// ```
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CollisionEvent {
    /// The two entities started to collide
    Started(Entity, Entity),

    /// The two entities no longer collide
    Stopped(Entity, Entity),
}

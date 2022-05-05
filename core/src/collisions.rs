use bevy_ecs::prelude::*;
use bevy_reflect::prelude::*;
use bevy_utils::HashMap;

use crate::{CollisionData, CollisionEvent, RigidBody};

/// Component which will be filled (if present) with a list of entities with which the current entity is currently in contact.
#[derive(Component, Default, Reflect)]
pub struct Collisions(HashMap<Entity, CollisionData>);

impl Collisions {
    /// Returns the number of colliding entities.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns `true` if there is no colliding entities.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns `true` if the collisions contains the specified entity.
    #[must_use]
    pub fn contains(&self, entity: &Entity) -> bool {
        self.0.contains_key(entity)
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    #[deprecated(note = "Please use `entities()` instead")]
    #[doc(hidden)]
    pub fn iter(&self) -> impl Iterator<Item = Entity> + '_ {
        self.entities()
    }

    /// An iterator visiting all colliding entities in arbitrary order.
    pub fn entities(&self) -> impl Iterator<Item = Entity> + '_ {
        self.0.keys().copied()
    }

    /// An iterator visiting all data from colliding entities in arbitrary order.
    pub fn collision_data(&self) -> impl Iterator<Item = &CollisionData> + '_ {
        self.0.values()
    }
}

/// Adds entity to [`CollidingEntities`] on starting collision and removes from it when the
/// collision end.
pub(super) fn update_collisions_system(
    mut collision_events: EventReader<'_, '_, CollisionEvent>,
    mut collisions: Query<'_, '_, &mut Collisions>,
) {
    for event in collision_events.iter() {
        let (data1, data2) = event.clone().data();
        let (entity1, entity2) = (data1.rigid_body_entity(), data2.rigid_body_entity());
        if event.is_started() {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.insert(entity2, data2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.insert(entity1, data1);
            }
        } else {
            if let Ok(mut entities) = collisions.get_mut(entity1) {
                entities.0.remove(&entity2);
            }
            if let Ok(mut entities) = collisions.get_mut(entity2) {
                entities.0.remove(&entity1);
            }
        }
    }
}

/// Removes deleted entities from [`Collisions`] component because
/// entity deletion doesn't emit [`CollisionEvent::Stopped`].
/// It's an upstream [issue](https://github.com/dimforge/rapier/issues/299).
pub(super) fn cleanup_collisions_system(
    removed_rigid_bodies: RemovedComponents<'_, RigidBody>,
    mut collisions: Query<'_, '_, &mut Collisions>,
) {
    for rigid_body in removed_rigid_bodies.iter() {
        for mut colliding_entities in collisions.iter_mut() {
            colliding_entities.0.remove(&rigid_body);
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy_app::App;
    use bevy_ecs::event::Events;

    use crate::{CollisionData, CollisionLayers};

    use super::*;

    #[test]
    fn collisions_updates() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>()
            .add_system(update_collisions_system);

        let entity1 = app.world.spawn().insert(Collisions::default()).id();
        let entity2 = app.world.spawn().insert(Collisions::default()).id();

        let collision_data1 =
            CollisionData::new(entity1, Entity::from_raw(0), CollisionLayers::default(), []);
        let collision_data2 =
            CollisionData::new(entity2, Entity::from_raw(0), CollisionLayers::default(), []);
        let mut collision_events = app.world.resource_mut::<Events<CollisionEvent>>();
        collision_events.send(CollisionEvent::Started(
            collision_data1.clone(),
            collision_data2.clone(),
        ));

        app.update();

        let collisions1 = app.world.entity(entity1).get::<Collisions>().unwrap();
        assert_eq!(collisions1.len(), 1, "There should be one colliding entity");
        assert_eq!(
            collisions1.entities().next().unwrap(),
            entity2,
            "Colliding entity should be equal to the second entity"
        );

        assert_eq!(
            collisions1.collision_data().next().unwrap(),
            &collision_data2,
            "Colliding entity data should be equal to the second collision data"
        );

        let collisions2 = app.world.entity(entity2).get::<Collisions>().unwrap();
        assert_eq!(collisions2.len(), 1, "There should be one colliding entity");
        assert_eq!(
            collisions2.entities().next().unwrap(),
            entity1,
            "Colliding entity should be equal to the first entity"
        );

        assert_eq!(
            collisions2.collision_data().next().unwrap(),
            &collision_data1,
            "Colliding entity data should be equal to the first collision data"
        );

        let mut collision_events = app.world.resource_mut::<Events<CollisionEvent>>();
        collision_events.send(CollisionEvent::Stopped(collision_data1, collision_data2));

        app.update();

        let collisions1 = app.world.entity(entity1).get::<Collisions>().unwrap();
        assert!(
            collisions1.is_empty(),
            "Colliding entity should be removed from the Collisions component when the collision ends"
        );

        let collisions2 = app.world.entity(entity2).get::<Collisions>().unwrap();
        assert!(
            collisions2.is_empty(),
            "Colliding entity should be removed from the Collisions component when the collision ends"
        );
    }

    #[test]
    fn collisions_react_on_entity_removal() {
        let mut app = App::new();
        app.add_event::<CollisionEvent>()
            .add_system(cleanup_collisions_system);

        let removing_entity = app.world.spawn().insert(RigidBody::Static).id();
        let mut collisions = Collisions::default();
        collisions.0.insert(
            removing_entity,
            CollisionData::new(
                removing_entity,
                Entity::from_raw(0),
                CollisionLayers::default(),
                [],
            ),
        );
        let entity = app.world.spawn().insert(collisions).id();

        app.update();

        app.world.entity_mut(removing_entity).despawn();

        app.update();

        let collisions = app.world.entity(entity).get::<Collisions>().unwrap();
        assert!(
            collisions.is_empty(),
            "Despawned entity should be removed from the Collisions component"
        );
    }
}

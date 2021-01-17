use bevy_ecs::*;
use bevy_transform::components::GlobalTransform;

use heron_core::Body;

use crate::rapier::dynamics::{RigidBodyBuilder, RigidBodySet};
use crate::rapier::geometry::{ColliderBuilder, ColliderSet};
use crate::BodyHandle;

pub(crate) fn create(
    commands: &mut Commands,
    mut bodies: ResMut<RigidBodySet>,
    mut colliders: ResMut<ColliderSet>,
    query: Query<(Entity, &Body, &GlobalTransform), Without<BodyHandle>>,
) {
    for (entity, body, _) in query.iter() {
        let rigid_body = bodies.insert(RigidBodyBuilder::new_dynamic().build());
        let collider = colliders.insert(collider_builder(body).build(), rigid_body, &mut bodies);
        commands.insert_one(
            entity,
            BodyHandle {
                rigid_body,
                collider,
            },
        );
    }
}

fn collider_builder(body: &Body) -> ColliderBuilder {
    match body {
        Body::Sphere { radius } => ColliderBuilder::ball(*radius),
    }
}

use bevy_ecs::prelude::*;

use heron_core::PhysicsMaterial;

use crate::rapier::geometry::ColliderSet;
use crate::BodyHandle;

pub(crate) fn update_rapier_restitution(
    mut colliders: ResMut<'_, ColliderSet>,
    restitutions: Query<'_, (&BodyHandle, &PhysicsMaterial), Changed<PhysicsMaterial>>,
) {
    for (handle, material) in restitutions.iter() {
        if let Some(collider) = colliders.get_mut(handle.collider) {
            collider.restitution = material.restitution;
        }
    }
}

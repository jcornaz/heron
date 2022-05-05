use std::collections::LinkedList;

use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::{CollisionLayers, CollisionShape, RigidBody};

/// Component which indicates that this entity or its children contains meshes which waiting for collision generation.
///
/// Once a mesh is added (for example, Bevy loads the GTLF scenes asynchronously), then the entity or its children will be added [`RigidBody`] and [`CollisionShape::ConvexHull`] (based on the geometry) components.
///
/// # Example
///
/// ```
/// # use bevy::prelude::*;
/// # use heron_core::*;
/// fn spawn(mut commands: Commands, asset_server: ResMut<AssetServer>) {
///     commands
///         .spawn()
///         .insert(Transform::default()) // Required to apply GLTF transforms in Bevy
///         .insert(GlobalTransform::default())
///         .insert(PendingConvexCollision::default())
///         .insert(RigidBody::Static)
///         .insert(CollisionLayers::default())
///         .with_children(|parent| {
///             parent.spawn_scene(asset_server.load("cubes.glb#Scene0"));
///         });
/// }
/// ```
#[derive(Component, Clone, Copy, Reflect)]
pub struct PendingConvexCollision {
    /// Rigid body type which will be assigned to every scene entity.
    #[deprecated(note = "Insert body type component into the entity with this component")]
    #[doc(hidden)]
    pub body_type: RigidBody,
    /// Border radius that will be used for [`CollisionShape::ConvexHull`].
    pub border_radius: Option<f32>,
}

#[allow(deprecated)]
impl Default for PendingConvexCollision {
    fn default() -> Self {
        Self {
            body_type: RigidBody::Static,
            border_radius: None,
        }
    }
}

/// Generates collision and attaches physics body for all entities with [`PendingConvexCollision`].
#[allow(deprecated)]
#[allow(clippy::type_complexity)] // Do not warn about long query
pub(super) fn pending_collision_system(
    mut commands: Commands<'_, '_>,
    added_scenes: Query<
        '_,
        '_,
        (
            Entity,
            &Children,
            &PendingConvexCollision,
            Option<&RigidBody>,
            Option<&CollisionLayers>,
        ),
    >,
    scene_elements: Query<'_, '_, &Children, Without<PendingConvexCollision>>,
    mesh_handles: Query<'_, '_, &Handle<Mesh>>,
    meshes: Option<Res<'_, Assets<Mesh>>>,
) {
    let meshes = match meshes {
        None => return,
        Some(m) => m,
    };
    for (scene, children, pending_collision, rigid_body, collision_layers) in added_scenes.iter() {
        let children = recursive_scene_children(children, &scene_elements);
        for child in &children {
            if let Ok(handle) = mesh_handles.get(*child) {
                let mesh = meshes.get(handle).unwrap(); // SAFETY: Mesh already loaded
                let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
                    VertexAttributeValues::Float32x3(vertices) => vertices,
                    _ => unreachable!(
                        "Mesh should have encoded vertices as VertexAttributeValues::Float32x3"
                    ),
                };
                let mut points = Vec::with_capacity(vertices.len());
                for vertex in vertices {
                    points.push(Vec3::new(vertex[0], vertex[1], vertex[2]));
                }
                let mut child_commands = commands.entity(*child);
                child_commands.insert(pending_collision.body_type).insert(
                    CollisionShape::ConvexHull {
                        points,
                        border_radius: pending_collision.border_radius,
                    },
                );
                if let Some(rigid_body) = rigid_body {
                    child_commands.insert(*rigid_body);
                }
                if let Some(collision_layers) = collision_layers {
                    child_commands.insert(*collision_layers);
                }
            }
        }
        if !children.is_empty() {
            commands
                .entity(scene)
                .remove::<PendingConvexCollision>()
                .remove::<RigidBody>()
                .remove::<CollisionLayers>();
        }
    }
}

/// Iterates over children hierarchy recursively and returns a plain list of all children.
/// [`LinkedList`] is used here for fast lists concatenation due to recursive iteration.
#[allow(clippy::linkedlist)]
fn recursive_scene_children(
    children: &Children,
    scene_elements: &Query<'_, '_, &Children, Without<PendingConvexCollision>>,
) -> LinkedList<Entity> {
    let mut all_children = LinkedList::new();
    for child in children.iter() {
        if let Ok(children) = scene_elements.get(*child) {
            let mut children = recursive_scene_children(children, scene_elements);
            all_children.append(&mut children);
        }
        all_children.push_back(*child);
    }
    all_children
}

#[cfg(test)]
mod tests {
    use bevy::{
        asset::AssetPlugin,
        core::CorePlugin,
        prelude::shape::{Capsule, Cube},
        render::{settings::WgpuSettings, RenderPlugin},
        window::WindowPlugin,
    };

    use super::*;

    // Allows run tests for systems containing rendering related things without GPU
    pub(super) struct HeadlessRenderPlugin;

    impl Plugin for HeadlessRenderPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(WgpuSettings {
                backends: None,
                ..WgpuSettings::default()
            })
            .add_plugin(CorePlugin::default())
            .add_plugin(WindowPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_plugin(RenderPlugin::default());
        }
    }

    #[test]
    fn dont_fail_without_render_plugin() {
        let mut app = App::new();
        app.add_system(pending_collision_system);
        app.update();
    }

    #[test]
    #[allow(deprecated)]
    fn pending_collision_assignes() {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_system(pending_collision_system);

        let mut meshes = app.world.resource_mut::<Assets<Mesh>>();
        let cube = meshes.add(Cube::default().into());
        let capsule = meshes.add(Capsule::default().into());

        const COLLISION_LAYERS: CollisionLayers = CollisionLayers::from_bits(1, 2);
        const BODY_TYPE: RigidBody = RigidBody::Static;
        const REQUESTED_COLLISION: PendingConvexCollision = PendingConvexCollision {
            body_type: RigidBody::Dynamic,
            border_radius: None,
        };

        let parent = app
            .world
            .spawn()
            .insert(REQUESTED_COLLISION)
            .insert(BODY_TYPE)
            .insert(COLLISION_LAYERS)
            .with_children(|parent| {
                parent.spawn().insert(cube);
                parent.spawn().insert(capsule);
            })
            .id();

        let mut query = app
            .world
            .query::<(&Handle<Mesh>, &RigidBody, &CollisionShape, &CollisionLayers)>();
        assert_eq!(
            query.iter(&app.world).count(),
            0,
            "Mesh handles, rigid bodies and collision shapes shouldn't exist before update"
        );

        app.update();

        assert_eq!(
            query.iter(&app.world).count(),
            2,
            "Entities with mesh handles should have rigid bodies and collision shapes after update"
        );

        let meshes = app.world.resource::<Assets<Mesh>>();
        for (mesh_handle, body_type, collision_shape, collision_layers) in query.iter(&app.world) {
            assert_eq!(
                *body_type, BODY_TYPE,
                "Assigned body type should be equal to specified"
            );

            let (points, border_radius) = match collision_shape {
                CollisionShape::ConvexHull {
                    points,
                    border_radius,
                } => (points, border_radius),
                _ => panic!("Assigned collision shape must be a convex hull"),
            };

            let mesh = meshes.get(mesh_handle).unwrap();
            let vertices = match mesh.attribute(Mesh::ATTRIBUTE_POSITION).unwrap() {
                VertexAttributeValues::Float32x3(vertices) => vertices,
                _ => unreachable!(
                    "Mesh should have encoded vertices as VertexAttributeValues::Float32x3"
                ),
            };
            for (point, vertex) in points.iter().zip(vertices) {
                assert_eq!(
                    point.x, vertex[0],
                    "x collision value should be equal to mesh vertex value"
                );
                assert_eq!(
                    point.y, vertex[1],
                    "y collision value should be equal to mesh vertex value"
                );
                assert_eq!(
                    point.z, vertex[2],
                    "z collision value should be equal to mesh vertex value"
                );
            }

            assert_eq!(
                *border_radius, REQUESTED_COLLISION.border_radius,
                "Assigned border radius should be equal to specified"
            );

            assert_eq!(
                *collision_layers, COLLISION_LAYERS,
                "Assigned collision layers should be equal to specified"
            );
        }

        assert!(
            !app.world
                .entity(parent)
                .contains::<PendingConvexCollision>(),
            "Parent entity should have PendingConvexCollision compoent removed"
        );
        assert!(
            !app.world.entity(parent).contains::<RigidBody>(),
            "Parent entity should have RigidBody compoent removed"
        );
        assert!(
            !app.world.entity(parent).contains::<CollisionLayers>(),
            "Parent entity should have CollisionLayers compoent removed"
        );
    }
}

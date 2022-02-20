use bevy::{prelude::*, render::mesh::VertexAttributeValues};

use crate::{CollisionShape, RigidBody};

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
///         .insert(PendingConvexCollision {
///             body_type: RigidBody::Static,
///             border_radius: None,
///         })
///         .with_children(|parent| {
///             parent.spawn_scene(asset_server.load("cubes.glb#Scene0"));
///         });
/// }
/// ```
#[derive(Component)]
pub struct PendingConvexCollision {
    /// Rigid body type which will be assigned to every scene entity.
    pub body_type: RigidBody,
    /// Border radius that will be used for [`CollisionShape::ConvexHull`].
    pub border_radius: Option<f32>,
}

/// Generates collision and attaches physics body for all entities with [`PendingConvexCollision`].
pub(super) fn pending_collision_system(
    mut commands: Commands<'_, '_>,
    added_scenes: Query<'_, '_, (Entity, &Children, &PendingConvexCollision)>,
    scene_elements: Query<'_, '_, &Children, Without<PendingConvexCollision>>,
    mesh_handles: Query<'_, '_, &Handle<Mesh>>,
    meshes: Option<Res<'_, Assets<Mesh>>>,
) {
    let meshes = match meshes {
        None => return,
        Some(m) => m,
    };
    for (entity, children, pending_collision) in added_scenes.iter() {
        if generate_collision(
            &mut commands,
            pending_collision,
            children,
            &scene_elements,
            &mesh_handles,
            &meshes,
        ) {
            // Only delete the component when the meshes are loaded and their is generated
            commands.entity(entity).remove::<PendingConvexCollision>();
        }
    }
}

/// Recursively generate collision and attach physics body for the specified children.
/// Returns `true` if a mesh was found.
fn generate_collision(
    commands: &mut Commands<'_, '_>,
    pending_collision: &PendingConvexCollision,
    children: &Children,
    scene_elements: &Query<'_, '_, &Children, Without<PendingConvexCollision>>,
    mesh_handles: &Query<'_, '_, &Handle<Mesh>>,
    meshes: &Assets<Mesh>,
) -> bool {
    let mut generated = false;
    for child in children.iter() {
        if let Ok(children) = scene_elements.get(*child) {
            if generate_collision(
                commands,
                pending_collision,
                children,
                scene_elements,
                mesh_handles,
                meshes,
            ) {
                generated = true;
            }
        }
        if let Ok(handle) = mesh_handles.get(*child) {
            generated = true;
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
            commands
                .entity(*child)
                .insert(pending_collision.body_type)
                .insert(CollisionShape::ConvexHull {
                    points,
                    border_radius: pending_collision.border_radius,
                });
        }
    }

    generated
}

#[cfg(test)]
mod tests {
    use bevy::{
        asset::AssetPlugin,
        core::CorePlugin,
        prelude::shape::{Capsule, Cube},
        render::{options::WgpuOptions, RenderPlugin},
        window::WindowPlugin,
    };

    use super::*;

    // Allows run tests for systems containing rendering related things without GPU
    pub(super) struct HeadlessRenderPlugin;

    impl Plugin for HeadlessRenderPlugin {
        fn build(&self, app: &mut App) {
            app.insert_resource(WgpuOptions {
                backends: None,
                ..Default::default()
            })
            .add_plugin(CorePlugin::default())
            .add_plugin(WindowPlugin::default())
            .add_plugin(AssetPlugin::default())
            .add_plugin(RenderPlugin::default());
        }
    }

    #[test]
    fn pending_collision_assignes() {
        let mut app = App::new();
        app.add_plugin(HeadlessRenderPlugin)
            .add_system(pending_collision_system);

        let mut meshes = app.world.get_resource_mut::<Assets<Mesh>>().unwrap();
        let cube = meshes.add(Cube::default().into());
        let capsule = meshes.add(Capsule::default().into());

        const REQUESTED_COLLISION: PendingConvexCollision = PendingConvexCollision {
            body_type: RigidBody::Static,
            border_radius: None,
        };

        let parent = app
            .world
            .spawn()
            .insert(REQUESTED_COLLISION)
            .with_children(|parent| {
                parent.spawn().insert(cube);
                parent.spawn().insert(capsule);
            })
            .id();

        let mut query = app
            .world
            .query::<(&Handle<Mesh>, &RigidBody, &CollisionShape)>();
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

        let meshes = app.world.get_resource::<Assets<Mesh>>().unwrap();
        for (mesh_handle, body_type, collision_shape) in query.iter(&app.world) {
            assert_eq!(
                *body_type, REQUESTED_COLLISION.body_type,
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
        }

        assert!(
            !app.world
                .entity(parent)
                .contains::<PendingConvexCollision>(),
            "Parent entity should have PendingConvexCollision removed"
        );
    }
}

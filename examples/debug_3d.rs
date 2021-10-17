use bevy::prelude::*;
use heron::prelude::*;

fn main() {
    App::new()
        .insert_resource(Msaa { samples: 4 })
        .insert_resource(Gravity::from(Vec3::new(0., -9.81, 0.)))
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_startup_system(setup.system())
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // HeightField
    commands
        .spawn_bundle((Transform::identity(), GlobalTransform::identity()))
        .insert(RigidBody::Static)
        .insert(CollisionShape::HeightField {
            size: Vec2::new(20., 20.),
            heights: vec![
                vec![1.5, 0.8, 0., 0., 3.0],
                vec![0.8, 0.2, 0., 0., 3.0],
                vec![0., 0.5, 0., 0., 3.0],
                vec![0., 0., 0.6, 0., 3.0],
                vec![3., 3., 3., 3., 3.0],
            ],
        });

    commands
        .spawn_bundle(PerspectiveCameraBundle::new_3d())
        .insert(
            Transform {
                translation: Vec3::new(3.0, 7., -19.0),
                ..Default::default()
            }
            .looking_at(Vec3::new(1., 4., 0.), Vec3::Y),
        );

    // Cube (with radius)
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::BLUE.into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(7.0, 15., 7.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.3, 0.3, 0.3),
            border_radius: Some(0.3),
        });

    // Cube (no radius)
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(Transform {
            translation: Vec3::new(0.0, 3.0, 0.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.5),
            border_radius: None,
        });

    // ConvexHull (no radius)
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube { size: 2.0 })),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(-2.0, 15., 8.0),
            ..Default::default()
        })
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::ConvexHull {
            points: vec![
                Vec3::new(-1.0, -1.0, -1.0),
                Vec3::new(-1.0, -1.0, 1.0),
                Vec3::new(-1.0, 1.0, -1.0),
                Vec3::new(1.0, -1.0, -1.0),
                Vec3::new(-1.0, 1.0, 1.0),
                Vec3::new(1.0, -1.0, 1.0),
                Vec3::new(1.0, 1.0, 1.0),
                Vec3::new(1.0, 1.0, -1.0),
                Vec3::new(0.0, 1.4, 0.0),
                Vec3::new(0.0, -1.4, 0.0),
                Vec3::new(0.0, 0.0, -1.6),
                Vec3::new(0.0, 0.0, 1.6),
            ],
            border_radius: None,
        });

    // Sphere
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere {
                radius: 1.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(3.0, 15., 3.0),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Sphere { radius: 1.0 });

    // Cylinder
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::GREEN.into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(3., 15., -7.),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cylinder {
            half_height: 1.0,
            radius: 0.5,
        });

    // Cone
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::RED.into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(5., 15., -7.),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Cone {
            half_height: 2.0,
            radius: 1.0,
        });

    // Capsule
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Capsule {
                radius: 0.5,
                depth: 2.0,
                ..Default::default()
            })),
            material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
            ..Default::default()
        })
        .insert(Transform {
            translation: Vec3::new(0., 15., 0.),
            ..Default::default()
        })
        .insert(GlobalTransform::identity())
        .insert(RigidBody::Dynamic)
        .insert(CollisionShape::Capsule {
            radius: 0.5,
            half_segment: 1.0,
        });

    // light
    commands.spawn_bundle(PointLightBundle {
        transform: Transform::from_xyz(-4.0, 9.0, -4.0),
        ..Default::default()
    });
}

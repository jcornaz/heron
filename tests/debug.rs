#![cfg(all(feature = "debug", feature = "2d"))]

use std::any::TypeId;

use bevy::core::CorePlugin;
use bevy::prelude::*;
use bevy::reflect::TypeRegistryArc;

use heron::*;

fn test_app() -> App {
    let mut builder = App::build();
    builder
        .init_resource::<TypeRegistryArc>()
        .add_plugin(CorePlugin)
        .add_plugin(PhysicsPlugin::with_steps_per_second(0));
    builder.app
}

#[test]
fn add_a_child_sprite() {
    let mut app = test_app();

    let parent = app
        .world
        .spawn((GlobalTransform::default(), Body::Sphere { radius: 12.0 }));

    app.update();

    let children = app
        .world
        .get::<Children>(parent)
        .expect("The body doesn't have any children");

    let mut iter = children.iter();
    let child = *iter
        .next()
        .expect("There is no child in the `Children` component");
    assert!(iter.next().is_none(), "Too many children found");

    assert!(app.world.has_component_type(child, TypeId::of::<Sprite>()));
    assert!(app
        .world
        .has_component_type(child, TypeId::of::<GlobalTransform>()));
    assert!(app.world.has_component_type(child, TypeId::of::<Draw>()));
    assert!(app
        .world
        .has_component_type(child, TypeId::of::<Handle<ColorMaterial>>()));
    assert!(app
        .world
        .has_component_type(child, TypeId::of::<Handle<Mesh>>()));

    assert!(app.world.get::<Visible>(child).unwrap().is_visible);
    assert_eq!(
        *app.world.get::<Transform>(child).unwrap(),
        Transform::default()
    );
}

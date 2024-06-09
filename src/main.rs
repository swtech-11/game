#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    app_with_render, app_without_render,
    config::ConfigRes,
    game_logic::entities::{
        creature::{Creature, Nutrition},
        fruit::Fruit,
    },
    render::controllable::Controllable,
};

fn main() {
    let mut app = app_without_render();
    // let mut app = app_with_render();
    app.world.spawn((
        Creature,
        Controllable,
        Nutrition(0),
        Collider::ball(1.0),
        RigidBody::Dynamic,
        ActiveEvents::COLLISION_EVENTS,
        TransformBundle {
            local: Transform::from_xyz(3.0, 3.0, 0.0),
            ..Default::default()
        },
    ));
    app.world.spawn((
        Fruit,
        Collider::cuboid(1.0, 1.0),
        RigidBody::Dynamic,
        ActiveEvents::COLLISION_EVENTS,
        TransformBundle {
            local: Transform::from_xyz(10.0, 10.0, 0.0),
            ..Default::default()
        },
    ));
    app.insert_resource(ConfigRes {
        bounds: Vec2::new(20.0, 20.0),
    });
    app.run();
}

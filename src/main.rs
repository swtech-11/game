#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    app_with_render, app_without_render,
    game_logic::entities::{
        creature::{Creature, Nutrition},
        fruit::Fruit,
    },
};

fn main() {
    // app_without_render().run();
    let mut app = app_with_render();
    app.world.spawn((
        Creature,
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
            local: Transform::from_xyz(3.0, 3.0, 0.0),
            ..Default::default()
        },
    ));
    app.run();
}

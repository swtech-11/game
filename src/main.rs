#![allow(unused_imports)]
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    config::ConfigRes,
    game_logic::entities::{
        creature::{Creature, CreatureBundle, Nutrition},
        fruit::{Fruit, FruitBundle},
    },
    render::controllable::Controllable,
};

fn main() {
    _fruit_creature_run();
}

fn _fruit_creature_run() {
    let mut app = game::app();

    app.insert_resource(ConfigRes {
        bounds: (Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)),
    });

    app.world.spawn(CreatureBundle {
        creature: Creature,
        nutrition: Nutrition(0),
        collider: Collider::ball(1.0),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        impulse: ExternalImpulse {
            impulse: Vec2::new(1.0, 0.0),
            ..Default::default()
        },
    });
    app.world.spawn(FruitBundle {
        fruit: Fruit,
        collider: Collider::cuboid(1.0, 1.0),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(10.0, 10.0, 0.0),
            ..Default::default()
        },
    });

    app.run();
}

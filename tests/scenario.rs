use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    config::ConfigRes,
    game_logic::entities::{
        creature::{Creature, CreatureBundle, Nutrition},
        fruit::{Fruit, FruitBundle},
    },
};

extern crate game;

#[test]
fn scenario() {
    let creature = CreatureBundle {
        creature: Creature,
        nutrition: Nutrition(0),
        collider: Collider::ball(1.0),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(3.0, 3.0, 0.0),
            ..Default::default()
        },
    };
    let fruit = FruitBundle {
        fruit: Fruit,
        collider: Collider::cuboid(1.0, 1.0),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(10.0, 10.0, 0.0),
            ..Default::default()
        },
    };
    let config = ConfigRes {
        bounds: Vec2::new(20.0, 20.0),
    };

    // let mut app = game::app_without_render();
    // app.world.spawn(creature.clone());
    // app.world.spawn(fruit.clone());
    // app.insert_resource(config.clone());
    // app.update();

    let mut app_render = game::app_with_render();
    app_render.world.spawn(creature);
    app_render.world.spawn(fruit);
    app_render.insert_resource(config);
    app_render.update();

    // app.world.query::<((), With<Creature>)>();
    // dbg!(app.world);
}

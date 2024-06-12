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
        impulse: ExternalImpulse::default(),
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

    let mut app = game::app_without_render();
    let creature_entity = app.world.spawn(creature).id();
    app.world.spawn(fruit);
    app.insert_resource(config);
    app.update();

    {
        let creature_ref = app.world.get_entity(creature_entity).unwrap();
        let transform = creature_ref.get::<Transform>().unwrap();
        dbg!(transform.translation);
    }
    app.update();

    {
        let mut creature_ref = app.world.get_entity_mut(creature_entity).unwrap();
        {
            let mut impulse = creature_ref.get_mut::<ExternalImpulse>().unwrap();
            impulse.impulse = Vec2::new(10.0, 0.0);
        }
        let transform = creature_ref.get::<Transform>().unwrap();
        dbg!(transform.translation);
    }
    app.update();

    {
        let creature_ref = app.world.get_entity(creature_entity).unwrap();
        let transform = creature_ref.get::<Transform>().unwrap();
        dbg!(transform.translation);
    }
    app.update();

    {
        let creature_ref = app.world.get_entity(creature_entity).unwrap();
        let transform = creature_ref.get::<Transform>().unwrap();
        dbg!(transform.translation);
    }
    app.update();

    {
        let creature_ref = app.world.get_entity(creature_entity).unwrap();
        let transform = creature_ref.get::<Transform>().unwrap();
        dbg!(transform.translation);
    }
}

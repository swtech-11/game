use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use game::{
    config::ConfigRes,
    game_logic::entities::{
        creature::{Creature, CreatureBundle, Nutrition},
        fruit::{Fruit, FruitBundle},
    },
};

fn main() {
    let mut app: App;
    #[cfg(feature = "render")]
    {
        app = game::app_with_render();
        debug!("App with render");
    }
    #[cfg(not(feature = "render"))]
    {
        app = game::app_without_render();
        debug!("App without render");
    }

    app.insert_resource(ConfigRes {
        bounds: (Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)),
    });

    app.world.spawn(CreatureBundle {
        creature: Creature,
        nutrition: Nutrition(0),
        collider: Collider::cuboid(0.5, 0.5),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(0.0, 0.0, 0.0),
            ..Default::default()
        },
        velocity: Velocity::default(),
        impulse: ExternalImpulse {
            impulse: Vec2::new(3.0, 0.0),
            ..Default::default()
        },
    });
    app.world.spawn(FruitBundle {
        fruit: Fruit,
        collider: Collider::cuboid(0.5, 0.5),
        rigid_body: RigidBody::Dynamic,
        active_events: ActiveEvents::COLLISION_EVENTS,
        transform: TransformBundle {
            local: Transform::from_xyz(3.0, 0.0, 0.0),
            ..Default::default()
        },
    });

    app.run();
}

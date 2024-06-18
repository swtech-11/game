use bevy::prelude::*;
use game::{
    config::ConfigRes,
    game_logic::entities::{creature::CreatureBundle, fruit::FruitBundle},
    render::controllable::Controllable,
    rng::rand_float,
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

    let config = ConfigRes {
        bounds: (Vec2::new(-20.0, -20.0), Vec2::new(20.0, 20.0)),
    };

    app.insert_resource(config.clone());

    app.world
        .spawn(CreatureBundle {
            transform: TransformBundle {
                local: Transform::from_xyz(
                    rand_float(config.bounds.0.x, config.bounds.1.x),
                    rand_float(config.bounds.0.y, config.bounds.1.y),
                    0.0,
                ),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Controllable);
    app.world.spawn(FruitBundle {
        transform: TransformBundle {
            local: Transform::from_xyz(
                rand_float(config.bounds.0.x, config.bounds.1.x),
                rand_float(config.bounds.0.y, config.bounds.1.y),
                0.0,
            ),
            ..Default::default()
        },
        ..Default::default()
    });

    app.run();
}

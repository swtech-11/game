mod game_logic;
mod render;
use bevy::prelude::*;
use env_logger;

use game_logic::{CreatureBundle, FruitBundle, GameLogicPlugin, Position};

fn main() {
    env_logger::init();

    let mut app = App::new();
    app.add_plugins(GameLogicPlugin).add_systems(Startup, setup);

    #[cfg(feature = "render")]
    {
        use render::RenderPlugin;
        app.add_plugins((DefaultPlugins, RenderPlugin));
    }
    #[cfg(not(feature = "render"))]
    {
        use bevy::input::InputPlugin;
        app.add_plugins((MinimalPlugins, InputPlugin));
    }

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn(CreatureBundle::new(Position { x: 5, y: 5 }));
    commands.spawn(FruitBundle::new(Position { x: 10, y: 10 }));
}

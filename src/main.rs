mod game_logic;
mod render;
use bevy::prelude::*;

use game_logic::{spawn_creature, GameLogicPlugin, Position};
use render::RenderPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(GameLogicPlugin).add_systems(Startup, setup);

    #[cfg(feature = "render")]
    app.add_plugins((DefaultPlugins, RenderPlugin));
    #[cfg(not(feature = "render"))]
    app.add_plugins(MinimalPlugins);

    app.run();
}

fn setup(commands: Commands) {
    spawn_creature(commands, Position { x: 0, y: 0 })
}

use bevy::{log::LogPlugin, prelude::*};
use game_logic::GameLogicPlugin;
use state::StatePlugin;

mod config;
mod game_logic;
mod render;
mod rng;
mod state;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(LogPlugin::default())
        .add_plugins(StatePlugin);

    #[cfg(feature = "render")]
    app.add_plugins(render::RenderPlugin);

    app.run()
}

use bevy::{log::LogPlugin, prelude::*};
use game_logic::GameLogicPlugin;

mod config;
mod game_logic;
mod render;
mod rng;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(LogPlugin::default());

    #[cfg(feature = "render")]
    app.add_plugins(render::RenderPlugin);

    app.run()
}

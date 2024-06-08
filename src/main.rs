use bevy::prelude::*;
use game_logic::GameLogicPlugin;
use log::CustomLogPlugin;
use state::StatePlugin;

mod config;
mod game_logic;
mod log;
mod render;
mod rng;
mod state;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(CustomLogPlugin)
        .add_plugins(StatePlugin);

    #[cfg(feature = "render")]
    app.add_plugins(render::RenderPlugin);

    app.run()
}

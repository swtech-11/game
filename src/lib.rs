use bevy::prelude::*;
use game_logic::GameLogicPlugin;
use log::CustomLogPlugin;
use state::StatePlugin;

pub mod config;
pub mod game_logic;
mod log;
pub mod render;
mod rng;
mod state;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(CustomLogPlugin)
        .add_plugins(StatePlugin);
    app
}

pub fn app_with_render() -> App {
    let mut app = app();
    app.add_plugins(render::RenderPlugin);
    app
}

pub fn app_without_render() -> App {
    app()
}

use bevy::prelude::*;
use game_logic::GameLogicPlugin;
use logs::CustomLogPlugin;
use physics_engine::PhysicsEnginePlugin;
use render::RenderPlugin;

pub mod config;
pub mod game_logic;
mod logs;
mod physics_engine;
pub mod render;
pub mod rng;

fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(CustomLogPlugin)
        .add_plugins(PhysicsEnginePlugin);
    app
}

pub fn app_with_render() -> App {
    let mut app = app();
    app.add_plugins(RenderPlugin);
    app
}

pub fn app_without_render() -> App {
    app()
}

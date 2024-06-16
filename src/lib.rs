use bevy::prelude::*;
use game_logic::GameLogicPlugin;
use log::CustomLogPlugin;
use physics_engine::PhysicsEnginePlugin;
use state::StatePlugin;

pub mod config;
pub mod game_logic;
mod log;
mod physics_engine;
pub mod render;
mod rng;
mod state;

pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins)
        .add_plugins(GameLogicPlugin)
        .add_plugins(CustomLogPlugin)
        .add_plugins(StatePlugin)
        .add_plugins(PhysicsEnginePlugin)
        .add_plugins(render::RenderPlugin);
    app
}

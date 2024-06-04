use bevy::prelude::*;
use fps::FPSPlugin;
use game_logic::GameLogicPlugin;

mod fps;
mod game_logic;
mod render;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins).add_plugins(GameLogicPlugin);
    // .add_plugins(FPSPlugin);

    #[cfg(feature = "render")]
    {
        app.add_plugins(render::RenderPlugin);
    }

    app.run()
}

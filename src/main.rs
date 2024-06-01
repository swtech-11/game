mod game_logic;
mod logger;
mod render;

use bevy::{log::LogPlugin, prelude::*};

use game_logic::GameLogicPlugin;

fn main() {
    logger::init();

    let mut app = App::new();
    app.add_plugins(GameLogicPlugin);

    #[cfg(feature = "render")]
    {
        use render::RenderPlugin;
        app.add_plugins(DefaultPlugins.build().disable::<LogPlugin>());
        app.add_plugins(RenderPlugin);
    }
    #[cfg(not(feature = "render"))]
    {
        use bevy::input::InputPlugin;
        app.add_plugins((MinimalPlugins, InputPlugin));
    }

    app.run();
}

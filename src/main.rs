mod game_logic;
mod render;
use bevy::prelude::*;

use game_logic::GameLogicPlugin;

fn main() {
    let mut app = App::new();
    app.add_plugins(GameLogicPlugin);

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

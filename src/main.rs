use bevy::prelude::*;
use creature::CreaturePlugin;
use fps::FPSPlugin;

use physics::PhysicsPlugin;

mod camera;
mod creature;
mod fps;
mod gizmos;
mod persistent_window;
mod physics;

fn main() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CreaturePlugin)
        .add_plugins(FPSPlugin);

    #[cfg(feature = "render")]
    {
        use bevy::time::TimePlugin;
        use bevy_inspector_egui::quick::WorldInspectorPlugin;
        use camera::CameraPlugin;
        use gizmos::GizmosPlugin;
        use persistent_window::PersistentWindowPlugin;

        let default_plugins = DefaultPlugins
            .build()
            .disable::<WindowPlugin>()
            .disable::<TaskPoolPlugin>()
            .disable::<TypeRegistrationPlugin>()
            .disable::<FrameCountPlugin>()
            .disable::<TimePlugin>();
        app.add_plugins(default_plugins)
            .add_plugins(PersistentWindowPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(GizmosPlugin)
            .add_plugins(WorldInspectorPlugin::new());
    }

    app.run()
}

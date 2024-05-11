mod config;
mod custom_debug;
use bevy::prelude::*;
use bevy_framepace::FramepacePlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use custom_debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins((
            #[cfg(debug_assertions)]
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                ..default()
            }),
            DebugPlugin,
            #[cfg(not(debug_assertions))]
            DefaultPlugins,
            FramepacePlugin,
            PanCamPlugin::default(),
        ))
        .init_resource::<Game>()
        .add_systems(Startup, setup)
        .run();
}

#[derive(Resource, Default)]
struct Game {
    entities: Vec<Entity>,
}

fn setup(mut commands: Commands, mut settings: ResMut<bevy_framepace::FramepaceSettings>) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(60.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

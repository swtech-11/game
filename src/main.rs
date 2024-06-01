use bevy::prelude::*;
use camera::CameraPlugin;
use persistent_window::PersistentWindowPlugin;
use physics::PhysicsPlugin;

mod camera;
mod persistent_window;
mod physics;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: None,
                    ..Default::default()
                })
                .build(),
        )
        .add_plugins(PersistentWindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .run();
}

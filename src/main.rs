use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use creature::CreaturePlugin;
use persistent_window::PersistentWindowPlugin;
use physics::PhysicsPlugin;

mod camera;
mod creature;
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
        .add_plugins(CreaturePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .run();
}

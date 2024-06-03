use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use creature::CreaturePlugin;
use fps::FPSPlugin;
use gizmos::GizmosPlugin;
use persistent_window::PersistentWindowPlugin;
use physics::PhysicsPlugin;

mod camera;
mod creature;
mod fps;
mod gizmos;
mod persistent_window;
mod physics;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.build().disable::<WindowPlugin>())
        .add_plugins(PersistentWindowPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PhysicsPlugin)
        .add_plugins(CreaturePlugin)
        .add_plugins(WorldInspectorPlugin::new())
        .add_plugins(GizmosPlugin)
        .add_plugins(FPSPlugin)
        .run();
}

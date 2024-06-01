use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanCamPlugin::default())
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    // Add a camera so we can see the debug-render.
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

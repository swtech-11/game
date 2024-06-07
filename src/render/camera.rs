use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

use crate::config::{BOUNDS_X, BOUNDS_Y};

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
        .spawn(Camera2dBundle {
            projection: OrthographicProjection {
                scale: 0.05,
                ..default()
            },
            transform: Transform::from_xyz(BOUNDS_X / 2.0, BOUNDS_Y / 2.0, 0.0),
            ..default()
        })
        .insert(PanCam::default());
}

use crate::game_logic::Position;
mod debug;

// MaterialMesh2dBundle {
//     mesh,
//     material,
//     transform: Transform::from_scale(Vec3 {
//         x: (10.0),
//         y: (10.0),
//         z: (0.0),
//     }),
//     ..default()
// },

// meshes.add(Rectangle::default()).into(),
//         materials.add(Color::PURPLE),

use bevy::prelude::*;
use bevy_pancam::{PanCam, PanCamPlugin};

pub const CELL_SIZE: f32 = 10.0;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing RenderPlugin");

        app.add_plugins((PanCamPlugin::default(),))
            .add_systems(Startup, setup)
            .add_systems(Update, position_sync);

        #[cfg(feature = "debug")]
        {
            use debug::RenderDebugPlugin;
            app.add_plugins(RenderDebugPlugin);
        }
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn position_sync(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.0);
    }
}

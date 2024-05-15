use crate::game_logic::{Creature, Position};
use log;
mod debug;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_pancam::{PanCam, PanCamPlugin};

pub const CELL_SIZE: f32 = 10.0;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        log::info!("Initializing RenderPlugin");

        #[cfg(feature = "debug")]
        {
            use debug::RenderDebugPlugin;
            app.add_plugins(RenderDebugPlugin);
        }

        app.add_plugins((PanCamPlugin::default(),))
            .add_systems(Startup, setup)
            .add_systems(PostStartup, create_render)
            .add_systems(Update, position_sync);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn create_render(
    mut commands: Commands,
    entity: Query<Entity, With<Creature>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    let mesh_bundle = MaterialMesh2dBundle {
        mesh: meshes.add(Rectangle::default()).into(),
        material: materials.add(Color::PURPLE),
        transform: Transform::from_scale(Vec3 {
            x: (10.0),
            y: (10.0),
            z: (0.0),
        }),
        ..default()
    };

    for entity in entity.iter() {
        dbg!(entity);
        commands.entity(entity).insert(mesh_bundle.clone());
    }
}

fn position_sync(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.0);
    }
}

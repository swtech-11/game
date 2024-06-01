use crate::game_logic::{Creature, Position};
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
            .add_systems(Update, creature_render)
            .add_systems(Update, creature_position);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());
}

fn creature_render(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Creature>>,

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

    for (entity, position) in query.iter() {
        log::debug!("Spawning creature");
        let mut mesh = mesh_bundle.clone();
        mesh.transform = Transform::from_translation(Vec3::new(
            position.x as f32 * CELL_SIZE,
            position.y as f32 * CELL_SIZE,
            0.0,
        ));
        commands.entity(entity).insert(mesh);
    }
}

fn creature_position(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        let mut final_position: (f32, f32) = (0.0, 0.0);
        final_position.0 = position.x as f32 * CELL_SIZE + CELL_SIZE / 2.0;
        final_position.1 = position.y as f32 * CELL_SIZE + CELL_SIZE / 2.0;

        transform.translation = Vec3::new(final_position.0, final_position.1, 0.0);
    }
}

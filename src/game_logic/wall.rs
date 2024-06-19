use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::ConfigRes;

pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, config: Res<ConfigRes>) {
    const WALL_THICKNESS: f32 = 1.0;
    let min_x = config.bounds.0.x;
    let max_x = config.bounds.1.x;
    let min_y = config.bounds.0.y;
    let max_y = config.bounds.1.y;

    commands
        .spawn(Sensor)
        .insert(TransformBundle::default())
        .with_children(|children| {
            // Left Wall
            children
                .spawn(Collider::cuboid(WALL_THICKNESS, (max_y - min_y) / 2.0))
                .insert(Name::new("Wall Left"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    min_x - WALL_THICKNESS,
                    (max_y + min_y) / 2.0,
                    0.0,
                )));

            // Right Wall
            children
                .spawn(Collider::cuboid(WALL_THICKNESS, (max_y - min_y) / 2.0))
                .insert(Name::new("Wall Right"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    max_x + WALL_THICKNESS,
                    (max_y + min_y) / 2.0,
                    0.0,
                )));

            // Bottom Wall
            children
                .spawn(Collider::cuboid((max_x - min_x) / 2.0, WALL_THICKNESS))
                .insert(Name::new("Wall Bottom"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    (max_x + min_x) / 2.0,
                    min_y - WALL_THICKNESS,
                    0.0,
                )));

            // Top Wall
            children
                .spawn(Collider::cuboid((max_x - min_x) / 2.0, WALL_THICKNESS))
                .insert(Name::new("Wall Top"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    (max_x + min_x) / 2.0,
                    max_y + WALL_THICKNESS,
                    0.0,
                )));
        });
}

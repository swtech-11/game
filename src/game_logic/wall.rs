use crate::config::{BOUNDS_X, BOUNDS_Y};
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
pub struct WallPlugin;

impl Plugin for WallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Sensor)
        .insert(TransformBundle::default())
        .with_children(|children| {
            children
                .spawn(Collider::cuboid(1.0, BOUNDS_Y / 2.0))
                .insert(Name::new("Wall Left"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    0.0 - 1.0,
                    BOUNDS_Y / 2.0,
                    0.0,
                )));
            children
                .spawn(Collider::cuboid(1.0, BOUNDS_Y / 2.0))
                .insert(Name::new("Wall Right"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    BOUNDS_X + 1.0,
                    BOUNDS_Y / 2.0,
                    0.0,
                )));
            children
                .spawn(Collider::cuboid(BOUNDS_X / 2.0, 1.0))
                .insert(Name::new("Wall Bottom"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    BOUNDS_X / 2.0,
                    0.0 - 1.0,
                    0.0,
                )));
            children
                .spawn(Collider::cuboid(BOUNDS_X / 2.0, 1.0))
                .insert(Name::new("Wall Top"))
                .insert(TransformBundle::from(Transform::from_xyz(
                    BOUNDS_X / 2.0,
                    BOUNDS_Y + 1.0,
                    0.0,
                )));
        });
}

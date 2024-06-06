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
                .spawn(Collider::cuboid(1.0, 30.0))
                .insert(TransformBundle::from(Transform::from_xyz(-40.0, 0.0, 0.0)));
            children
                .spawn(Collider::cuboid(1.0, 30.0))
                .insert(TransformBundle::from(Transform::from_xyz(40.0, 0.0, 0.0)));
            children
                .spawn(Collider::cuboid(40.0, 1.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, -30.0, 0.0)));
            children
                .spawn(Collider::cuboid(40.0, 1.0))
                .insert(TransformBundle::from(Transform::from_xyz(0.0, 30.0, 0.0)));
        });
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Fruit;

pub struct FruitPlugin;

impl Plugin for FruitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Collider::ball(1.0))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(TransformBundle {
            local: Transform {
                translation: Vec3 {
                    x: 10.0,
                    y: 0.0,
                    z: 0.0,
                },
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ReadMassProperties::default())
        .insert(Fruit);
}

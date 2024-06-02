use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, movement);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(1.0, 1.0))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(TransformBundle::default())
        .insert(ReadMassProperties::default());
}

fn movement(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<RigidBody>>,
    mut commands: Commands,
) {
    if input.just_pressed(KeyCode::ArrowUp) {
        for body in query.iter() {
            commands.entity(body).insert(ExternalImpulse {
                impulse: Vec2::new(1.0, 0.0),
                ..Default::default()
            });
        }
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::rng::{in_bounds_rng, rand_float};

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Nutrition(pub u8);

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        // .add_systems(Update, movement);

        #[cfg(feature = "render")]
        app.add_systems(Update, input);
    }
}

fn setup(mut commands: Commands) {
    commands
        .spawn(Collider::cuboid(1.0, 1.0))
        .insert(RigidBody::Dynamic)
        .insert(Velocity::default())
        .insert(in_bounds_rng())
        .insert(ReadMassProperties::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Nutrition(0))
        .insert(Name::new("Creature"))
        .insert(Creature);
}

fn movement(query: Query<Entity, With<Creature>>, mut commands: Commands) {
    for body in query.iter() {
        let impulse = Vec2::new(rand_float(-10.0, 10.0), rand_float(-10.0, 10.0));
        commands.entity(body).insert(ExternalImpulse {
            impulse,
            ..Default::default()
        });
    }
}

fn input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Creature>>,
    mut commands: Commands,
) {
    for body in query.iter() {
        let mut impulse = Vec2::ZERO;

        if keyboard_input.just_pressed(KeyCode::ArrowLeft) {
            impulse.x -= 10.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowRight) {
            impulse.x += 10.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            impulse.y += 10.0;
        }
        if keyboard_input.just_pressed(KeyCode::ArrowDown) {
            impulse.y -= 10.0;
        }

        commands.entity(body).insert(ExternalImpulse {
            impulse,
            ..Default::default()
        });
    }
}

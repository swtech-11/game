use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::rng::in_bounds_rng;

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Nutrition(pub u8);

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
        .insert(in_bounds_rng())
        .insert(ReadMassProperties::default())
        .insert(ActiveEvents::COLLISION_EVENTS)
        .insert(Nutrition(0))
        .insert(Creature);
}

fn movement(query: Query<Entity, With<Creature>>, mut commands: Commands) {
    for body in query.iter() {
        commands.entity(body).insert(ExternalImpulse {
            impulse: Vec2::new(1.0, 0.0),
            ..Default::default()
        });
    }
}

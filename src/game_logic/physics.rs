use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drag);
    }
}

fn drag(mut query: Query<&mut Velocity>) {
    for mut velocity in &mut query {
        velocity.linvel *= 0.99;
    }
}

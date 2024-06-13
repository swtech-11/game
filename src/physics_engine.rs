use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsEnginePlugin;

impl Plugin for PhysicsEnginePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.0));
    }
}

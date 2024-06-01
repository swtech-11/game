use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(0.0))
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}

// fn setup_physics(mut commands: Commands) {
//     /* Create the ground. */
//     commands
//         .spawn(Collider::cuboid(500.0, 50.0))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, -100.0, 0.0)));

//     /* Create the bouncing ball. */
//     commands
//         .spawn(RigidBody::Dynamic)
//         .insert(Collider::ball(50.0))
//         .insert(Restitution::coefficient(0.7))
//         .insert(TransformBundle::from(Transform::from_xyz(0.0, 400.0, 0.0)));
// }

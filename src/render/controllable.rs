use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone)]
pub struct Controllable;

pub fn control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &mut ExternalImpulse), With<Controllable>>,
    mut commands: Commands,
) {
    for mut entity in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            entity.0.rotate(Quat::from_rotation_z(0.05));
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            entity.0.rotate(Quat::from_rotation_z(-0.05));
        }
        if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            let rotation = entity.0.rotation;
            let forward = rotation.mul_vec3(Vec3::X);
            entity.1.impulse = forward.xy() * 1.0;
        }
    }
}

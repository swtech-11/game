use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone)]
pub struct Controllable;

pub fn control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Controllable>>,
    mut commands: Commands,
) {
    for entity in query.iter() {
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

        commands.entity(entity).insert(ExternalImpulse {
            impulse,
            ..Default::default()
        });
    }
}

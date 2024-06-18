use bevy::prelude::*;

use crate::game_logic::entities::creature::{Action, ActionState};

#[derive(Component, Clone)]
pub struct Controllable;

pub fn control(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ActionState, With<Controllable>>,
) {
    for mut action_state in query.iter_mut() {
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            action_state.current_action = Some(Action::TurnLeft);
        } else if keyboard_input.pressed(KeyCode::ArrowRight) {
            action_state.current_action = Some(Action::TurnRight);
        } else if keyboard_input.just_pressed(KeyCode::ArrowUp) {
            action_state.current_action = Some(Action::MoveForward);
        }
    }
}

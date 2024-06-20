use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use super::entities::{creature::Creature, fruit::Fruit};

mod dqn;
mod replay_buffer;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decision);
    }
}

#[derive(Clone)]
struct CreatureState {
    velocity: Velocity,
    transform: Transform,
    distance_to_fruit: f32,
}

fn decision(
    mut creature_query: Query<(&mut Transform, &Velocity), With<Creature>>,
    fruit_query: Query<&Transform, (With<Fruit>, Without<Creature>)>,
) {
    for (mut creature_transform, creature_velocity) in creature_query.iter_mut() {
        let creature_position = creature_transform.translation.truncate();
        let mut state = CreatureState {
            velocity: *creature_velocity,
            transform: *creature_transform,
            distance_to_fruit: f32::INFINITY,
        };

        // For now, it will just go to the closest fruit
        for fruit_transform in fruit_query.iter() {
            let fruit_position = fruit_transform.translation.truncate();
            let distance = state
                .transform
                .translation
                .truncate()
                .distance(fruit_position);
            if distance < state.distance_to_fruit {
                state.distance_to_fruit = distance;
            }
        }

        // Now that we have the "CreatureState", we need to use it to find the best decision and update ActionState.
        // To create the DQN, maybe it would be ok to a create the DQN as a component. That way, we could query any DQN with creature.
        // But first, let's commit this tomorrow and implement a way to reset the environment and train.
    }
}

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use dqn::QNetwork;
use rand::Rng;

use super::entities::{
    creature::{Creature, CreatureAction},
    fruit::Fruit,
};

pub mod dqn;
pub mod replay_buffer;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decision);
    }
}

const ALPHA: f32 = 0.001;
const GAMMA: f32 = 0.99;
const EPSILON: f32 = 0.1;
const BATCH_SIZE: usize = 32;

fn decision(
    mut creature_query: Query<(&mut Transform, &Velocity, &mut QNetwork), With<Creature>>,
    fruit_query: Query<&Transform, (With<Fruit>, Without<Creature>)>,
) {
    let mut rng = rand::thread_rng();
    for (mut creature_transform, creature_velocity, mut q_network) in creature_query.iter_mut() {
        // Find the closest fruit
        let mut closest_fruit_position = Vec2::ZERO;
        let mut distance_to_fruit = f32::INFINITY;
        for fruit_transform in fruit_query.iter() {
            let fruit_position = fruit_transform.translation.truncate();
            let distance = creature_transform
                .translation
                .truncate()
                .distance(fruit_position);
            if distance < distance_to_fruit {
                distance_to_fruit = distance;
                closest_fruit_position = fruit_position;
            }
        }

        // Define the current state
        let state = vec![
            creature_velocity.linvel.x,
            creature_velocity.linvel.y,
            distance_to_fruit,
            creature_transform.translation.x,
            creature_transform.translation.y,
        ];

        // Choose an action based on the epsilon-greedy policy
        let action = if rng.gen::<f32>() < EPSILON {
            rng.gen_range(0..4) // Explore: random action
        } else {
            let q_values = q_network.forward(state.clone());
            q_values
                .iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0
        };

        // Map the action index to a CreatureAction
        let creature_action: CreatureAction = match action {
            0 => CreatureAction::MoveForward,
            1 => CreatureAction::TurnLeft,
            2 => CreatureAction::TurnRight,
            3 => CreatureAction::None,
            _ => CreatureAction::None,
        };

        // Apply the action
        match creature_action {
            CreatureAction::MoveForward => {
                // Move the creature forward
                let forward_direction = creature_transform.rotation * Vec3::Y;
                creature_transform.translation += forward_direction * 0.1; // Move forward by 0.1 units
            }
            CreatureAction::TurnLeft => {
                // Rotate the creature left
                creature_transform.rotate(Quat::from_rotation_z(0.1)); // Rotate by 0.1 radians
            }
            CreatureAction::TurnRight => {
                // Rotate the creature right
                creature_transform.rotate(Quat::from_rotation_z(-0.1)); // Rotate by -0.1 radians
            }
            CreatureAction::None => {
                // Do nothing
            }
        }

        // Calculate the reward based on the new distance to the closest fruit
        let new_distance_to_fruit = creature_transform
            .translation
            .truncate()
            .distance(closest_fruit_position);
        let reward = if new_distance_to_fruit < distance_to_fruit {
            1.0 // Reward for getting closer to the fruit
        } else {
            -0.1 // Penalty for not getting closer to the fruit
        };

        // Define the next state
        let next_state = vec![
            creature_velocity.linvel.x,
            creature_velocity.linvel.y,
            new_distance_to_fruit,
            creature_transform.translation.x,
            creature_transform.translation.y,
        ];

        // Store the experience in the replay buffer
        q_network.add_experience((state, action, reward, next_state));

        // Train the Q-network
        q_network.train(BATCH_SIZE, ALPHA, GAMMA);
    }
}

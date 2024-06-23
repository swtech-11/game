use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use creature_state::CreatureState;
use dqn::QNetwork;
use persistency::{load_qnetwork_from_file, save_periodically};
use rand::Rng;

use super::entities::{
    creature::{ActionState, Creature, CreatureAction},
    fruit::Fruit,
};

mod creature_state;
pub mod dqn;
mod persistency;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_creature_brains)
            .add_systems(Update, decision)
            .add_systems(PostUpdate, save_periodically);
    }
}

// For now, this is okayish because is temporary and I don't know how the final creatures will look like
fn load_creature_brains(mut commands: Commands, creature_query: Query<Entity, With<Creature>>) {
    let dqns = load_qnetwork_from_file();

    if dqns.is_empty() {
        warn!("No brains found in the brain directory. This means you are starting with a blank slate.");
        return;
    }

    let mut count = 0;
    for entity in creature_query.iter() {
        if dqns.is_empty() {
            commands.entity(entity).insert(QNetwork::new(&[5, 24, 4]));
        }
        commands.entity(entity).insert(dqns[count].clone());
        count += 1;
    }
}

const ALPHA: f32 = 0.001;
const GAMMA: f32 = 0.99;
const EPSILON: f32 = 0.1;
const BATCH_SIZE: usize = 32;

fn decision(
    mut creature_query: Query<
        (&mut Transform, &Velocity, &mut QNetwork, &mut ActionState),
        With<Creature>,
    >,
    fruit_query: Query<&Transform, (With<Fruit>, Without<Creature>)>,
) {
    let mut rng = rand::thread_rng();
    for (creature_transform, creature_velocity, mut q_network, mut action_state) in
        creature_query.iter_mut()
    {
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
        let state = CreatureState {
            distance_to_fruit,
            creature_pos_x: creature_transform.translation.x,
            creature_pos_y: creature_transform.translation.y,
            creature_rot: creature_transform.rotation.z,
        };

        // Choose an action based on the epsilon-greedy policy
        let action = if rng.gen::<f32>() < EPSILON {
            rng.gen_range(0..4) // Explore: random action
        } else {
            let q_values = q_network.forward(state.to_vec());
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

        match creature_action {
            CreatureAction::MoveForward => {
                action_state.current_action = CreatureAction::MoveForward
            }
            CreatureAction::TurnLeft => action_state.current_action = CreatureAction::TurnLeft,
            CreatureAction::TurnRight => action_state.current_action = CreatureAction::TurnRight,
            CreatureAction::None => action_state.current_action = CreatureAction::None,
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
        q_network.add_experience((state.to_vec(), action, reward, next_state));

        // Train the Q-network
        q_network.train(BATCH_SIZE, ALPHA, GAMMA);
    }
}

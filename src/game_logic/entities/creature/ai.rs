use bevy::prelude::*;
// use serde::{Deserialize, Serialize};

// #[derive(Component, Serialize, Deserialize, Clone, Debug)]
// pub struct CreatureNetwork {
//     q_network: QNetwork<DenseLayer>,
//     creature_type: String,
// }

// impl CreatureNetwork {
//     pub fn new(creature_type: &str, layer_sizes: &[usize]) -> Self {
//         let q_network = QNetwork::<DenseLayer>::new(layer_sizes);
//         Self {
//             q_network,
//             creature_type: creature_type.to_string(),
//         }
//     }

//     pub fn forward(&mut self, inputs: Vec<f32>) -> Vec<f32> {
//         self.q_network.forward(inputs)
//     }

//     pub fn train(&mut self, batch_size: usize, learning_rate: f32, gamma: f32) {
//         self.q_network.train(batch_size, learning_rate, gamma);
//     }

//     pub fn add_experience(&mut self, experience: (Vec<f32>, usize, f32, Vec<f32>)) {
//         self.q_network.add_experience(experience);
//     }

//     pub fn get_creature_type(&self) -> &str {
//         &self.creature_type
//     }
// }

pub fn decision(
    mut creature_query: Query<(&mut Transform, &mut QNetwork, &mut ActionState), With<Creature>>,
    fruit_query: Query<&Transform, (With<Fruit>, Without<Creature>)>,
) {
    let mut rng = rand::thread_rng();
    for (creature_transform, mut q_network, mut action_state) in creature_query.iter_mut() {
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

        let next_state = CreatureState {
            distance_to_fruit,
            creature_pos_x: creature_transform.translation.x,
            creature_pos_y: creature_transform.translation.y,
            creature_rot: creature_transform.rotation.z,
        };

        // Store the experience in the replay buffer
        q_network.add_experience((state.to_vec(), action, reward, next_state.to_vec()));

        // Train the Q-network
        q_network.train(BATCH_SIZE, ALPHA, GAMMA);
    }
}

pub struct CreatureState {
    pub distance_to_fruit: f32,
    pub creature_pos_x: f32,
    pub creature_pos_y: f32,
    pub creature_rot: f32,
}

impl CreatureState {
    pub fn to_vec(&self) -> Vec<f32> {
        vec![
            self.distance_to_fruit,
            self.creature_pos_x,
            self.creature_pos_y,
            self.creature_rot,
        ]
    }
}

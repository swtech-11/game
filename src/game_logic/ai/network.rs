use bevy::prelude::*;
use layer::Layer;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::fmt::Debug;

mod layer;

#[derive(Component, Serialize, Deserialize, Clone, Debug)]
pub struct QNetwork<L: Layer> {
    layers: Vec<L>,
    #[serde(skip)] // Skip replay buffer in serialization
    replay_buffer: Vec<(Vec<f32>, usize, f32, Vec<f32>)>,
}

impl<L: Layer> QNetwork<L> {
    pub fn new(layer_sizes: &[usize]) -> Self {
        let layers = layer_sizes.windows(2).map(|w| L::new(w[0], w[1])).collect();
        Self {
            layers,
            replay_buffer: Vec::new(),
        }
    }

    pub fn forward(&mut self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter_mut()
            .fold(inputs, |input, layer| layer.forward(input))
    }

    fn backward(&mut self, gradients: Vec<f32>, learning_rate: f32) {
        let _ = self
            .layers
            .iter_mut()
            .rev()
            .fold(gradients, |grad, layer| layer.backward(grad, learning_rate));
    }

    pub fn add_experience(&mut self, experience: (Vec<f32>, usize, f32, Vec<f32>)) {
        self.replay_buffer.push(experience);
        if self.replay_buffer.len() > 1000 {
            self.replay_buffer.remove(0);
        }
    }

    fn sample_experiences(&self, batch_size: usize) -> Vec<(Vec<f32>, usize, f32, Vec<f32>)> {
        let mut rng = rand::thread_rng();
        (0..batch_size)
            .map(|_| {
                let index = rng.gen_range(0..self.replay_buffer.len());
                self.replay_buffer[index].clone()
            })
            .collect()
    }

    pub fn train(&mut self, batch_size: usize, learning_rate: f32, gamma: f32) {
        if self.replay_buffer.len() < batch_size {
            return;
        }

        let batch = self.sample_experiences(batch_size);
        for (state, action, reward, next_state) in batch {
            let current_q_values = self.forward(state.clone());
            let next_q_values = self.forward(next_state.clone());
            let target_q_value =
                reward + gamma * next_q_values.iter().cloned().fold(f32::MIN, f32::max);

            let mut gradients = vec![0.0; current_q_values.len()];
            gradients[action] = current_q_values[action] - target_q_value;

            self.backward(gradients, learning_rate);
        }
    }
}

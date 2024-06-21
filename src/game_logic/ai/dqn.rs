use bevy::prelude::*;
use rand::Rng;

#[derive(Clone)]
struct Layer {
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
    outputs: Vec<f32>,
    inputs: Vec<f32>,
    deltas: Vec<f32>,
}

impl Layer {
    fn new(input_size: usize, output_size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let weights = (0..output_size)
            .map(|_| {
                (0..input_size)
                    .map(|_| rng.gen::<f32>() * 2.0 - 1.0)
                    .collect()
            })
            .collect();
        let biases = (0..output_size)
            .map(|_| rng.gen::<f32>() * 2.0 - 1.0)
            .collect();
        Self {
            weights,
            biases,
            outputs: vec![0.0; output_size],
            inputs: vec![0.0; input_size],
            deltas: vec![0.0; output_size],
        }
    }

    fn forward(&mut self, inputs: Vec<f32>) -> Vec<f32> {
        self.inputs = inputs.clone();
        for i in 0..self.outputs.len() {
            self.outputs[i] = self.biases[i];
            for j in 0..self.inputs.len() {
                self.outputs[i] += self.inputs[j] * self.weights[i][j];
            }
            self.outputs[i] = self.outputs[i].max(0.0); // ReLU activation
        }
        self.outputs.clone()
    }

    fn backward(&mut self, gradients: Vec<f32>, learning_rate: f32) -> Vec<f32> {
        let mut input_gradients = vec![0.0; self.inputs.len()];
        for i in 0..self.outputs.len() {
            self.deltas[i] = gradients[i] * (self.outputs[i] > 0.0) as i32 as f32;
            for j in 0..self.inputs.len() {
                input_gradients[j] += self.deltas[i] * self.weights[i][j];
                self.weights[i][j] -= learning_rate * self.deltas[i] * self.inputs[j];
            }
            self.biases[i] -= learning_rate * self.deltas[i];
        }
        input_gradients
    }
}

#[derive(Component)]
pub struct QNetwork {
    layers: Vec<Layer>,
    replay_buffer: Vec<(Vec<f32>, usize, f32, Vec<f32>)>,
}

impl QNetwork {
    pub fn new(sizes: &[usize]) -> Self {
        let layers = sizes.windows(2).map(|w| Layer::new(w[0], w[1])).collect();
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

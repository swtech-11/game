use rand::Rng;
use std::fmt::Debug;

pub trait Layer: Send + Sync + Debug {
    fn forward(&mut self, inputs: Vec<f32>) -> Vec<f32>;
    fn backward(&mut self, gradients: Vec<f32>, learning_rate: f32) -> Vec<f32>;
}

#[derive(Debug)]
pub struct DenseLayer {
    weights: Vec<Vec<f32>>,
    biases: Vec<f32>,
    outputs: Vec<f32>,
    inputs: Vec<f32>,
    deltas: Vec<f32>,
}

impl DenseLayer {
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
}

impl Layer for DenseLayer {
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

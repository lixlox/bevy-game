use bevy_inspector_egui::Inspectable;
use rand::random;
use serde::{Deserialize, Serialize};

#[derive(Inspectable, Default, Clone, Serialize, Deserialize)]
pub struct Level {
    inputs: Vec<f32>,
    outputs: Vec<f32>,
    biases: Vec<f32>,
    weights: Vec<Vec<f32>>,
}

impl Level {
    pub fn new(input_count: usize, output_count: usize) -> Level {
        let mut level = Level {
            biases: vec![0.; output_count],
            inputs: vec![0.; input_count],
            outputs: vec![0.; output_count],
            weights: vec![vec![0.; output_count]; input_count],
        };
        level.randomize_level();
        level
    }

    fn randomize_level(&mut self) {
        self.weights = self
            .weights
            .iter()
            .map(|weight_vec| {
                weight_vec
                    .iter()
                    .map(|_weight| random::<f32>() * 2. - 1.)
                    .collect()
            })
            .collect();

        self.biases = self
            .biases
            .iter()
            .map(|_bias| random::<f32>() * 2. - 1.)
            .collect();
    }

    pub fn feed_forward(&mut self, given_inputs: Vec<f32>) -> Vec<f32> {
        self.inputs = given_inputs;
        for i in 0..self.outputs.len() {
            let mut sum = 0.;
            for j in 0..self.inputs.len() {
                sum += self.inputs[j] * self.weights[j][i];
            }

            if sum > self.biases[i] {
                self.outputs[i] = 1.;
            } else {
                self.outputs[i] = 0.;
            }
        }

        self.outputs.clone()
    }

    pub fn mutate(&mut self, amount: f32) {
        self.weights = self
            .weights
            .iter()
            .map(|weight| {
                weight
                    .iter()
                    .map(|w| lerp(*w, random::<f32>() * 2. - 1., amount))
                    .collect::<Vec<f32>>()
            })
            .collect();

        self.biases = self
            .biases
            .iter()
            .map(|bias| lerp(*bias, random::<f32>() * 2. - 1., amount))
            .collect();
    }
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    (1. - t) * v0 + t * v1
}

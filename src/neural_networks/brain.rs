use super::level::Level;
use bevy::prelude::Component;
use bevy_inspector_egui::Inspectable;
use serde::{Deserialize, Serialize};

#[derive(Component, Inspectable, Default, Clone, Serialize, Deserialize)]
pub struct NeuralNetwork {
    #[inspectable()]
    pub levels: Vec<Level>,
}

impl NeuralNetwork {
    pub fn new(neurons_count: Vec<usize>) -> NeuralNetwork {
        let mut levels = Vec::new();

        for i in 0..neurons_count.len() - 1 {
            levels.push(Level::new(neurons_count[i], neurons_count[i + 1]));
        }

        NeuralNetwork { levels }
    }

    pub fn feed_forward(&mut self, given_inputs: Vec<f32>) -> Vec<f32> {
        let mut outputs = self.levels[0].feed_forward(given_inputs);

        for i in 1..self.levels.len() {
            outputs = self.levels[i].feed_forward(outputs);
        }

        outputs
    }

    pub fn mutate(&mut self, amount: f32) {
        self.levels
            .iter_mut()
            .for_each(|level| level.mutate(amount));
    }
}

use super::brain::NeuralNetwork;

#[derive(Clone)]
pub struct Generation {
    pub neural_networks: Vec<NeuralNetwork>,
    pub generation_number: u32,
}

impl Generation {
    pub fn new() -> Generation {
        Generation {
            neural_networks: Vec::new(),
            generation_number: 0,
        }
    }
}

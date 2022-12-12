use bevy::prelude::Entity;

use crate::neural_networks::brain::NeuralNetwork;

pub struct PlayerDieEvent(pub Entity);

pub struct SpawnPlayers {
    pub number: u32,
    pub neural_network: Option<NeuralNetwork>,
}

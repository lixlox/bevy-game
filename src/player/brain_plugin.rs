use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

use bevy::prelude::{
    Commands, Entity, EventReader, EventWriter, Plugin, Query, Res, ResMut, Transform, With,
};

use crate::{
    components::{Collider, Pipe, Velocity},
    neural_networks::{brain::NeuralNetwork, generation::Generation},
    WinSize, BASE_SPEED,
};

use super::{
    components::Player,
    events::{PlayerDieEvent, SpawnPlayers},
};

pub struct BrainPlugin;

impl Plugin for BrainPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_generation_add_player_system)
            .add_system(player_neural_network_feed_forward_system)
            .add_system(player_mutate_on_generation_die_system);
    }
}

fn player_mutate_on_generation_die_system(
    mut generations: ResMut<Generation>,
    query: Query<Entity, With<Player>>,
    mut writer: EventWriter<SpawnPlayers>,
    query_collision: Query<Entity, With<Collider>>,
    mut commands: Commands,
) {
    if query.iter().len() == 0 {
        if let Some(neural_network) = generations.neural_networks.last() {
            if Path::new("neural_network_save.json").exists() {
                fs::remove_file("neural_network_save.json").unwrap();
            }

            let mut file = File::create("neural_network_save.json").unwrap();

            file.write_all(serde_json::to_string(neural_network).unwrap().as_bytes())
                .unwrap();

            writer.send(SpawnPlayers {
                number: 500,
                neural_network: Some(neural_network.clone()),
            });

            generations.generation_number += 1;
            generations.neural_networks = Vec::new();
        }

        for entity in query_collision.iter() {
            commands.entity(entity).despawn();
        }
    }
}

fn player_generation_add_player_system(
    mut reader: EventReader<PlayerDieEvent>,
    query: Query<(Entity, &NeuralNetwork), With<Player>>,
    mut commands: Commands,
    mut generations: ResMut<Generation>,
) {
    let player_die_entities: Vec<Entity> = reader
        .iter()
        .map(|player_die_event| player_die_event.0)
        .collect();
    for (entity, neural_network) in query.iter() {
        if player_die_entities.contains(&entity) {
            generations.neural_networks.push(neural_network.clone());
            commands.entity(entity).despawn();
        }
    }
}

fn player_neural_network_feed_forward_system(
    win_size: Res<WinSize>,
    mut query: Query<(&mut NeuralNetwork, &mut Velocity, &Transform), With<Player>>,
    pipes_query: Query<&Transform, With<Pipe>>,
) {
    for (mut neural_network, mut velocity, transform) in query.iter_mut() {
        let mut sorted_pipes: Vec<&Transform> = pipes_query.iter().collect();

        let player_position = transform.translation.y;

        let output = if sorted_pipes.len() >= 2 {
            sorted_pipes.sort_by(|a, b| a.translation.x.partial_cmp(&b.translation.x).unwrap());
            neural_network.feed_forward(vec![
                player_position / (win_size.h / 2.),
                ((sorted_pipes[0].translation.y + sorted_pipes[1].translation.y) / 2.)
                    / (win_size.h / 2.),
                sorted_pipes[0].translation.x / (win_size.w / 2.),
            ])[0]
                == 1.
        } else {
            neural_network.feed_forward(vec![player_position / (win_size.h / 2.), 0., 0.])[0] == 1.
        };

        if output {
            velocity.y = 0.65 * BASE_SPEED;
        }
    }
}

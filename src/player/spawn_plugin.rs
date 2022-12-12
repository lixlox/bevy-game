use std::{fs, path::Path};

use bevy::{
    prelude::{Commands, EventReader, Plugin, Res, StartupStage, Transform, Vec3},
    sprite::SpriteSheetBundle,
    time::Timer,
};

use crate::{
    components::{AffectedByGravity, AnimationTimer, Velocity},
    neural_networks::brain::NeuralNetwork,
    GameTextures, PLAYER_SPRITE_SCALE,
};

use super::{
    components::{Player, Score},
    events::SpawnPlayers,
};

pub struct SpawnPlugin;

impl Plugin for SpawnPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, player_spawn_system)
            .add_system(player_spawn_handle_system);
    }
}

fn player_spawn_handle_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    mut reader: EventReader<SpawnPlayers>,
) {
    for spawn_players in reader.iter() {
        for i in 0..spawn_players.number {
            let neural_network = spawn_players.neural_network.clone();
            if let Some(mut nn) = neural_network {
                if i != 0 {
                    nn.mutate(0.10);
                }
                spawn_player(&mut commands, &game_textures, Some(nn));
            } else {
                spawn_player(&mut commands, &game_textures, None);
            }
        }
    }
}

fn player_spawn_system(mut commands: Commands, game_textures: Res<GameTextures>) {
    if Path::new("neural_network_save.json").exists() {
        let data =
            fs::read_to_string("neural_network_save.json").expect("Unable to read save file.");
        let neural_network =
            serde_json::from_str::<NeuralNetwork>(&data).expect("Unable to parse json file");
        spawn_player(&mut commands, &game_textures, Some(neural_network));
    } else {
        spawn_player(&mut commands, &game_textures, None);
    }
}

fn spawn_player(
    commands: &mut Commands,
    game_textures: &Res<GameTextures>,
    neural_network: Option<NeuralNetwork>,
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: game_textures.player.clone(),
            transform: Transform {
                translation: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 10.,
                },
                scale: Vec3 {
                    x: PLAYER_SPRITE_SCALE,
                    y: PLAYER_SPRITE_SCALE,
                    z: 1.,
                },

                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player)
        .insert(Velocity { x: 0., y: 0. })
        .insert(AffectedByGravity { is_affected: true })
        .insert(AnimationTimer {
            timer: Timer::from_seconds(0.1, true),
        })
        .insert(Score(0))
        .insert(neural_network.unwrap_or(NeuralNetwork::new(vec![3, 6, 1])));
}

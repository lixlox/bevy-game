use bevy::prelude::*;

use crate::{
    components::{AnimationTimer, Velocity},
    GameTextures, WinSize, BASE_SPEED, PLAYER_SIZE, PLAYER_SPRITE_SCALE,
};

use super::{
    brain_plugin::BrainPlugin, components::Player, events::PlayerDieEvent,
    movement_plugin::MovementPlugin, spawn_plugin::SpawnPlugin,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(MovementPlugin)
            .add_plugin(SpawnPlugin)
            .add_plugin(BrainPlugin)
            .add_system(player_keyboard_event_system)
            .add_system(player_animation_system)
            .add_system(check_player_border_overflow_system);
    }
}

fn player_keyboard_event_system(
    kb: Res<Input<KeyCode>>,
    mut query_player_velocity: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in query_player_velocity.iter_mut() {
        if kb.just_pressed(KeyCode::Space) {
            velocity.y = 0.65 * BASE_SPEED;
        }
    }
}

fn player_animation_system(
    time: Res<Time>,
    game_textures: Res<GameTextures>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite), With<Player>>,
) {
    for (mut timer, mut sprite) in query.iter_mut() {
        timer.timer.tick(time.delta());
        if timer.timer.just_finished() {
            let player_texture_atlas = texture_atlases.get(&game_textures.player).unwrap();
            sprite.index = (sprite.index + 1) % player_texture_atlas.textures.len();
        }
    }
}

fn check_player_border_overflow_system(
    win_size: Res<WinSize>,
    query: Query<(Entity, &Transform), With<Player>>,
    mut write: EventWriter<PlayerDieEvent>,
) {
    for (entity, transform) in query.iter() {
        let translation = transform.translation;
        if translation.y > win_size.h / 2. - PLAYER_SIZE.1 / 2. * PLAYER_SPRITE_SCALE
            || translation.y < -win_size.h / 2. + PLAYER_SIZE.1 / 2. * PLAYER_SPRITE_SCALE
        {
            write.send(PlayerDieEvent(entity));
        }
    }
}

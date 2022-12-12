use std::collections::HashSet;

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    components::Collider,
    player::{
        components::{Player, Score},
        events::PlayerDieEvent,
    },
    PIPE_SIZE, PIPE_SPRITE_SCALE, PLAYER_SIZE, PLAYER_SPRITE_SCALE,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(player_collision_system);
    }
}

fn player_collision_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, Entity, &mut Score), With<Player>>,
    collide_query: Query<(&Transform, &Collider, Entity), With<Collider>>,
    mut writer: EventWriter<PlayerDieEvent>,
) {
    let mut collider_to_despawn = HashSet::new();
    for (player_transform, player_entity, mut score) in player_query.iter_mut() {
        for (collide_transform, collide_collider, entity) in collide_query.iter() {
            match collide_collider {
                Collider::Loss => collide_loss(
                    player_transform,
                    collide_transform,
                    &mut writer,
                    player_entity,
                ),
                Collider::Win => {
                    collide_win(player_transform, collide_transform, &mut score);
                    collider_to_despawn.insert(entity);
                }
            }
        }
    }

    for entity in collider_to_despawn.iter() {
        commands.entity(*entity).despawn();
    }
}

fn collide_win(player_transform: &Transform, collide_transform: &Transform, score: &mut Score) {
    let collision = collide(
        player_transform.translation,
        Vec2::new(
            PLAYER_SIZE.0 * PLAYER_SPRITE_SCALE,
            PLAYER_SIZE.1 * PLAYER_SPRITE_SCALE,
        ),
        collide_transform.translation,
        Vec2::new(
            PLAYER_SIZE.0 * PLAYER_SPRITE_SCALE,
            PLAYER_SIZE.1 * PLAYER_SPRITE_SCALE,
        ),
    );

    if collision.is_some() {
        score.0 += 1;
    }
}

fn collide_loss(
    player_transform: &Transform,
    collide_transform: &Transform,
    writer: &mut EventWriter<PlayerDieEvent>,
    player_entity: Entity,
) {
    let collision = collide(
        player_transform.translation,
        Vec2::new(
            PLAYER_SIZE.0 * PLAYER_SPRITE_SCALE,
            PLAYER_SIZE.1 * PLAYER_SPRITE_SCALE,
        ),
        collide_transform.translation,
        Vec2::new(
            PIPE_SIZE.0 * PIPE_SPRITE_SCALE,
            PIPE_SIZE.1 * PIPE_SPRITE_SCALE,
        ),
    );

    if collision.is_some() {
        writer.send(PlayerDieEvent(player_entity));
    }
}

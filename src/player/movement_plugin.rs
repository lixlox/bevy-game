use bevy::{
    prelude::{Plugin, Query, Res, Transform, With},
    time::Time,
};

use crate::components::Velocity;

use super::components::Player;

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(player_movement_system);
    }
}

fn player_movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let delta = time.delta_seconds();
        translation.y += velocity.y * delta;
    }
}

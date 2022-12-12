use bevy::{prelude::Component, time::Timer};

use crate::GameStates;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Pipe;

#[derive(Component)]
pub struct AffectedByGravity {
    pub is_affected: bool,
}

#[derive(Component)]
pub struct AnimationTimer {
    pub timer: Timer,
}

#[derive(Component)]
pub enum Collider {
    Loss,
    Win,
}

#[derive(Component)]
pub struct TextGameState {
    pub state: GameStates,
}

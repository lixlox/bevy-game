use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use rand::{thread_rng, Rng};

use crate::{
    components::{Collider, Pipe, Velocity},
    GameTextures, PipeSpawnSettings, WinSize, BASE_SPEED, PIPE_SIZE, PIPE_SPRITE_SCALE,
};

pub struct PipePlugin;

impl Plugin for PipePlugin {
    fn build(&self, app: &mut App) {
        app.add_system(pipe_movement_system)
            .add_system(pipe_spawn_system)
            .add_system(pipe_despawn_system);
    }
}

fn pipe_despawn_system(
    mut commands: Commands,
    win_size: Res<WinSize>,
    query: Query<(&Transform, Entity), With<Collider>>,
) {
    for (transform, entity) in query.iter() {
        let translation = &transform.translation;
        if translation.x < (-win_size.w / 2. - PIPE_SIZE.0 * PIPE_SPRITE_SCALE) {
            commands.entity(entity).despawn();
        }
    }
}

fn pipe_movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform), With<Collider>>,
) {
    for (velocity, mut transform) in query.iter_mut() {
        let translation = &mut transform.translation;
        let delta = time.delta_seconds();
        translation.x += velocity.x * delta * BASE_SPEED;
    }
}

fn pipe_spawn_system(
    mut commands: Commands,
    time: Res<Time>,
    mut pipe_spawn_settings: ResMut<PipeSpawnSettings>,
    game_textures: Res<GameTextures>,
) {
    pipe_spawn_settings.timer.tick(time.delta());
    if pipe_spawn_settings.timer.just_finished() {
        let mut rng = thread_rng();
        let random_f32: f32 = rng.gen_range(-100. ..100.);
        spawn_pipe(&mut commands, game_textures, random_f32);
    }
}

fn spawn_pipe(commands: &mut Commands, game_textures: Res<GameTextures>, random_f32: f32) {
    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(game_textures.pipe_mesh.clone()),
            material: game_textures.pipe_material.clone(),
            transform: Transform {
                scale: Vec3::new(PIPE_SPRITE_SCALE, PIPE_SPRITE_SCALE, 0.0),
                translation: Vec3::new(1200., 300. + random_f32, 3.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Collider::Loss)
        .insert(Pipe)
        .insert(Velocity { x: -0.5, y: 0. });

    commands
        .spawn_bundle(MaterialMesh2dBundle {
            mesh: Mesh2dHandle(game_textures.pipe_mesh.clone()),
            material: game_textures.pipe_material.clone(),
            transform: Transform {
                scale: Vec3::new(PIPE_SPRITE_SCALE, PIPE_SPRITE_SCALE, 0.0),
                translation: Vec3::new(1200., -300. + random_f32, 3.),
                rotation: Quat::from_rotation_z(std::f32::consts::PI),
            },
            ..Default::default()
        })
        .insert(Collider::Loss)
        .insert(Pipe)
        .insert(Velocity { x: -0.5, y: 0. });

    commands.spawn_bundle((
        Transform {
            translation: Vec3::new(1200., random_f32, 0.),
            ..Default::default()
        },
        Collider::Win,
        Velocity { x: -0.5, y: 0. },
    ));
}

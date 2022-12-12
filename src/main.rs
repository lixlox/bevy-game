use bevy::{
    prelude::{shape::Box, *},
    render::texture::ImageSettings,
    window::PresentMode,
};
use collision::CollisionPlugin;
use gravity::GravityPlugin;
use neural_networks::generation::Generation;
use pipe::PipePlugin;
use player::{
    events::{PlayerDieEvent, SpawnPlayers},
    plugin::PlayerPlugin,
};
use textdisplay::TextDisplayPlugin;

mod collision;
mod components;
mod gravity;
mod neural_networks;
mod pipe;
mod player;
mod textdisplay;

const BASE_SPEED: f32 = 500.;

const PIPE_SPRITE: &str = "pipe.png";
const PIPE_SIZE: (f32, f32) = (32., 128.);
const PIPE_SPRITE_SCALE: f32 = 3.5;

const PLAYER_SPRITE: &str = "player-spritesheet.png";
const PLAYER_SIZE: (f32, f32) = (719., 612.);
const PLAYER_SPRITE_SCALE: f32 = 0.1;

struct WinSize {
    w: f32,
    h: f32,
}

struct GameTextures {
    player: Handle<TextureAtlas>,
    pipe_mesh: Handle<Mesh>,
    pipe_material: Handle<ColorMaterial>,
}

struct GameFont(Handle<Font>);

struct Gravity {
    amplitude: f32,
}

#[derive(PartialEq, Debug)]
pub enum GameStates {
    Playing,
    GameOver,
    StartScreen,
}

struct GameState {
    state: GameStates,
}

struct PipeSpawnSettings {
    timer: Timer,
}

// #[derive(Inspectable, Default)]
// struct Data {
//     query: InspectorQuery<Entity, With<NeuralNetwork>>,
// }

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.34, 0.75, 0.79)))
        .insert_resource(ImageSettings::default_nearest())
        .insert_resource(GameState {
            state: GameStates::Playing,
        })
        .insert_resource(PipeSpawnSettings {
            timer: Timer::from_seconds(3.0, true),
        })
        .insert_resource(Generation::new())
        .insert_resource(WindowDescriptor {
            title: "Flappy Rust".to_string(),
            width: 598.0,
            height: 676.0,
            present_mode: PresentMode::AutoVsync,
            ..Default::default()
        })
        .insert_resource(Gravity { amplitude: 3. })
        .add_event::<PlayerDieEvent>()
        .add_event::<SpawnPlayers>()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup_system)
        .add_system(win_size_refresh_system)
        .add_plugin(PlayerPlugin)
        .add_plugin(GravityPlugin)
        .add_plugin(TextDisplayPlugin)
        .add_plugin(PipePlugin)
        .add_plugin(CollisionPlugin)
        .run();
}

fn setup_system(
    mut commands: Commands,
    mut windows: ResMut<Windows>,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    //camera

    commands.spawn_bundle(Camera2dBundle::default());

    //capture window size

    let window = windows.get_primary_mut().unwrap();

    let (win_w, win_h) = (window.width(), window.height());

    // add WinSize ressource

    let win_size = WinSize { w: win_w, h: win_h };

    commands.insert_resource(win_size);
    let player_texture = asset_server.load(PLAYER_SPRITE);
    let player_texture_atlas = TextureAtlas::from_grid(
        player_texture,
        Vec2::new(PLAYER_SIZE.0, PLAYER_SIZE.1),
        2,
        2,
    );
    let player_texture_atlas_handle = texture_atlases.add(player_texture_atlas);
    let pipe_image_handle = asset_server.load(PIPE_SPRITE);
    let pipe_color_material = ColorMaterial {
        texture: Some(pipe_image_handle),
        ..Default::default()
    };
    let pipe_material_handle = materials.add(pipe_color_material);

    let mybox = Box::new(PIPE_SIZE.0, PIPE_SIZE.1, 1.);

    let mesh = Mesh::from(mybox);

    let mesh_handle = meshes.add(mesh);

    // add GameTextures ressource
    let game_textures = GameTextures {
        player: player_texture_atlas_handle,
        pipe_mesh: mesh_handle,
        pipe_material: pipe_material_handle,
    };

    commands.insert_resource(game_textures);

    let game_font = GameFont(asset_server.load("fonts/FiraSans-Bold.ttf"));

    commands.insert_resource(game_font);
}

fn win_size_refresh_system(mut win_size: ResMut<WinSize>, windows: Res<Windows>) {
    let window = windows.get_primary().unwrap();

    let (win_w, win_h) = (window.width(), window.height());

    win_size.h = win_h;
    win_size.w = win_w;
}

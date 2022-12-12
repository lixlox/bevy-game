use bevy::prelude::*;

use crate::{components::TextGameState, GameFont, GameState, GameStates};

pub struct TextDisplayPlugin;

impl Plugin for TextDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system_to_stage(StartupStage::PostStartup, text_display_startup_system)
            .add_system(text_display_system);
    }
}

fn text_display_startup_system(mut commands: Commands, game_font: Res<GameFont>) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                position_type: PositionType::Absolute,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::Column,
                ..Default::default()
            },
            color: UiColor(Color::Rgba {
                red: 0.,
                green: 0.,
                blue: 0.,
                alpha: 0.,
            }),
            ..Default::default()
        })
        .add_children(|parent| {
            playing_display(parent, game_font.0.clone());
            game_over_display(parent, game_font.0.clone());
            start_screen_display(parent, game_font.0.clone());
        });
}

fn text_display_system(
    game_state: Res<GameState>,
    mut query: Query<(&mut Text, &mut Visibility, &TextGameState), With<TextGameState>>,
    // query_score: Query<&Score, With<Player>>,
) {
    // let max_score = query_score.iter().map(|score| score.0).max();
    // let mut score_string: String = String::from("0");
    // if let Some(x) = max_score {
    //     score_string = x.to_string();
    // }
    for (mut text, mut visibility, text_game_state) in query.iter_mut() {
        if game_state.state != text_game_state.state {
            visibility.is_visible = false;
        } else {
            visibility.is_visible = true;
            text.sections[0].value = "".to_owned();
            // match text_game_state.state {
            //     // GameStates::Playing => text.sections[0].value = score_string.clone(),
            //     // GameStates::GameOver => {
            //     //     text.sections[0].value =
            //     //         "Game Over, Score : ".to_owned() + &score_string.as_str() + "\nPress Space";
            //     // }
            //     _ => text.sections[0].value = "".to_owned(),
            // }
        }
    }
}

fn playing_display(child_builder: &mut ChildBuilder, game_font: Handle<Font>) {
    child_builder
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "<Press Space To Start>",
                TextStyle {
                    font: game_font,
                    font_size: 50.,
                    color: Color::rgba(0., 0., 0., 0.3),
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                // position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }),
        )
        .insert(TextGameState {
            state: GameStates::Playing,
        });
}

fn game_over_display(child_builder: &mut ChildBuilder, game_font: Handle<Font>) {
    child_builder
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "<Press Space To Start>",
                TextStyle {
                    font: game_font,
                    font_size: 50.,
                    color: Color::BLACK,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                // position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }),
        )
        .insert(TextGameState {
            state: GameStates::GameOver,
        });
}

fn start_screen_display(child_builder: &mut ChildBuilder, game_font: Handle<Font>) {
    child_builder
        .spawn_bundle(
            TextBundle::from_section(
                // Accepts a `String` or any type that converts into a `String`, such as `&str`
                "<Press Space To Start>",
                TextStyle {
                    font: game_font,
                    font_size: 50.,
                    color: Color::BLACK,
                },
            ) // Set the alignment of the Text
            .with_text_alignment(TextAlignment::CENTER)
            // Set the style of the TextBundle itself.
            .with_style(Style {
                // position_type: PositionType::Relative,
                align_self: AlignSelf::Center,
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            }),
        )
        .insert(TextGameState {
            state: GameStates::StartScreen,
        });
}

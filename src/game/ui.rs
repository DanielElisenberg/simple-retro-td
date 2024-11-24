use crate::{
    constants::{SCREEN_SIZE_X, SCREEN_SIZE_Y},
    game::{
        components::{
            HealthText, LevelText, MobSpawner, MoneyText, OnGameScreen,
        },
        resources::Player,
    },
};
use bevy::{
    asset::Handle,
    color::Color,
    math::Vec3,
    prelude::{Commands, Query, Res, Transform, With, Without},
    text::{Font, Text, Text2dBundle, TextStyle},
    utils::default,
};

pub fn update_ui(
    mut text_query: Query<(
        &mut Text,
        Option<&LevelText>,
        Option<&HealthText>,
        Option<&MoneyText>,
    )>,
    mob_spawner: Query<&MobSpawner, With<MobSpawner>>,
    player: Res<Player>,
) {
    let current_level = mob_spawner.get_single().unwrap().current_level;
    let current_money = player.money;
    let current_life = player.life;

    for (mut text, level_text, health_text, money_text) in text_query.iter_mut()
    {
        if level_text.is_some() {
            text.sections[0].value = format!("{current_level}");
        }
        if health_text.is_some() {
            text.sections[0].value = format!("{current_life}");
        }
        if money_text.is_some() {
            text.sections[0].value = format!("{current_money}");
        }
    }
}

pub fn spawn_ui_entities(
    commands: &mut Commands,
    asset_server: &Res<bevy::asset::AssetServer>,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "1",
                TextStyle {
                    font: asset_server.load("fonts/tiny5/Tiny5-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3::new(
                    SCREEN_SIZE_X / 2. + 14.,
                    SCREEN_SIZE_Y - 12.,
                    1.,
                ),
                scale: Vec3::new(0.25, 0.25, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        LevelText,
        OnGameScreen,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "30",
                TextStyle {
                    font: asset_server.load("fonts/tiny5/Tiny5-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3::new(
                    (SCREEN_SIZE_X / 4.) * 3. + 32.,
                    10.,
                    1.,
                ),
                scale: Vec3::new(0.25, 0.25, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        HealthText,
        OnGameScreen,
    ));
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "30",
                TextStyle {
                    font: asset_server.load("fonts/tiny5/Tiny5-Regular.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3::new((SCREEN_SIZE_X / 4.) * 3., 10., 1.),
                scale: Vec3::new(0.25, 0.25, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        MoneyText,
        OnGameScreen,
    ));
}

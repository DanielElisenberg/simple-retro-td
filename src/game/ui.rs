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
    asset::Assets,
    color::Color,
    math::{UVec2, Vec3},
    prelude::{Commands, Query, Res, ResMut, Transform, With},
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    text::{Text, Text2dBundle, TextStyle},
    time::{Time, Timer, TimerMode},
    utils::default,
};

use super::components::{AnimationIndices, AnimationTimer, Coin, Heart};

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
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<bevy::asset::AssetServer>,
) {
    commands.spawn((
        Text2dBundle {
            text: Text::from_section(
                "1",
                TextStyle {
                    font: asset_server.load("fonts/tiny5/Tiny5-Regular.ttf"),
                    font_size: 50.0,
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
                scale: Vec3::new(0.2, 0.2, 1.),
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
                    font_size: 50.0,
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
                scale: Vec3::new(0.2, 0.2, 1.),
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
                    font_size: 50.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            transform: Transform {
                translation: Vec3::new((SCREEN_SIZE_X / 4.) * 3., 10., 1.),
                scale: Vec3::new(0.2, 0.2, 1.),
                ..Default::default()
            },
            ..Default::default()
        },
        MoneyText,
        OnGameScreen,
    ));
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("heart.png"),
            transform: Transform::from_xyz(SCREEN_SIZE_X - 26., 8., 1.),
            ..Default::default()
        },
        Heart,
        OnGameScreen,
    ));
    let spritesheet = asset_server.load("coin.png");
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(
                (SCREEN_SIZE_X / 4.) * 3. - 16.,
                10.,
                1.,
            ),
            texture: spritesheet,
            ..default()
        },
        TextureAtlas {
            layout: texture_atlas_layout,
            index: animation_indices.first,
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        Coin,
        OnGameScreen,
    ));
}

pub fn animate_coin(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas),
        With<Coin>,
    >,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

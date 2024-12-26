mod components;
mod control;
mod levels;
mod mobs;
mod projectiles;
mod resources;
mod towers;
mod ui;

use crate::{
    common::despawn_all,
    constants::{self, ALL_PATH_COORDINATES, SCREEN_SIZE_X, SCREEN_SIZE_Y},
    game::components::OnGameScreen,
    GameState,
};
use bevy::{
    app::{App, Update},
    asset::{AssetServer, Assets},
    input::ButtonInput,
    prelude::{
        in_state, Commands, IntoSystemConfigs, KeyCode, NextState, OnEnter,
        OnExit, Res, ResMut, Transform,
    },
    sprite::{SpriteBundle, TextureAtlasLayout},
};
use bevy_kira_audio::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_system)
        .insert_resource(resources::Player {
            life: 30,
            money: 30,
        })
        .insert_resource(resources::BlockList(Vec::from(
            constants::ALL_PATH_COORDINATES,
        )))
        .add_systems(
            Update,
            (
                change_scene,
                check_life,
                control::move_selector,
                mobs::ysort_mobs,
                mobs::move_mobs,
                mobs::spawn_mobs_from_spawner,
                mobs::animate_mobs,
                towers::animate_towers,
                towers::shoot_from_tower,
                projectiles::move_bullet_to_target,
                mobs::despawn_mobs,
                ui::update_ui,
                ui::animate_coin,
            )
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(
            OnExit(GameState::Game),
            (despawn_all::<OnGameScreen>, cleanup),
        );
}

fn setup_system(
    mut commands: Commands,
    audio: Res<Audio>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("embedded://sprites/backgrounds/game_screen.png"),
            transform: Transform::from_xyz(
                SCREEN_SIZE_X / 2.,
                SCREEN_SIZE_Y / 2.,
                0.,
            ),
            ..Default::default()
        },
        OnGameScreen,
    ));
    control::spawn_selector(&mut commands, &asset_server);
    mobs::init_mob_spawner(&mut commands);
    audio
        .play(asset_server.load("embedded://audio/bg_music.ogg"))
        .looped();
    ui::spawn_ui_entities(&mut commands, texture_atlas_layouts, &asset_server);
}

fn check_life(
    player: ResMut<resources::Player>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if player.life == 0 {
        game_state.set(GameState::Title);
    }
}

fn change_scene(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    audio: Res<Audio>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Title);
        audio.stop();
    }
}

fn cleanup(
    mut player: ResMut<resources::Player>,
    mut block_list: ResMut<resources::BlockList>,
) {
    player.life = 30;
    player.money = 30;
    block_list.0 = Vec::from(ALL_PATH_COORDINATES);
}

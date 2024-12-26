mod components;

use bevy::{
    app::{App, Update},
    asset::AssetServer,
    input::ButtonInput,
    prelude::{
        in_state, Commands, IntoSystemConfigs, KeyCode, NextState, OnEnter,
        OnExit, Res, ResMut, Transform,
    },
    sprite::SpriteBundle,
};
use bevy_kira_audio::prelude::*;

use crate::{
    common::despawn_all,
    constants::{SCREEN_SIZE_X, SCREEN_SIZE_Y},
    victory::components::OnVictoryScreen,
    GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Victory), setup_system)
        .add_systems(Update, change_scene.run_if(in_state(GameState::Victory)))
        .add_systems(
            OnExit(GameState::Victory),
            despawn_all::<OnVictoryScreen>,
        );
}

fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server
                .load("embedded://sprites/backgrounds/victory_screen.png"),
            transform: Transform::from_xyz(
                SCREEN_SIZE_X / 2.,
                SCREEN_SIZE_Y / 2.,
                0.,
            ),
            ..Default::default()
        },
        OnVictoryScreen,
    ));
    audio
        .play(asset_server.load("embedded://audio/title_music.ogg"))
        .looped();
}

fn change_scene(
    keys: Res<ButtonInput<KeyCode>>,
    audio: Res<Audio>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        audio.stop();
        game_state.set(GameState::Game)
    }
}

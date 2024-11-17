mod components;
mod control;
mod enemies;
mod projectiles;
mod towers;

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

use crate::{
    common::despawn_screen,
    constants::{SCREEN_SIZE_X, SCREEN_SIZE_Y},
    GameState, OnGameScreen,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_system)
        .add_systems(
            Update,
            (change_scene, control::move_selector)
                .run_if(in_state(GameState::Game)),
        )
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("game_screen.png"),
            transform: Transform::from_xyz(
                SCREEN_SIZE_X / 2.,
                SCREEN_SIZE_Y / 2.,
                0.,
            ),
            ..Default::default()
        },
        OnGameScreen,
    ));
    control::spawn_selector(&mut commands, &asset_server)
}

fn change_scene(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Title)
    }
}

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
        OnExit, Res, ResMut,
    },
    sprite::SpriteBundle,
};

use crate::{common::despawn_screen, GameState, OnGameScreen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Game), setup_system)
        .add_systems(Update, change_scene.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("game_screen.png"),
            ..Default::default()
        },
        OnGameScreen,
    ));
}

fn change_scene(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Title)
    }
}

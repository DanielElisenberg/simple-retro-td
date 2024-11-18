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

use crate::{
    common::despawn_all,
    constants::{SCREEN_SIZE_X, SCREEN_SIZE_Y},
    title::components::OnTitleScreen,
    GameState,
};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::Title), setup_system)
        .add_systems(Update, change_scene.run_if(in_state(GameState::Title)))
        .add_systems(OnExit(GameState::Title), despawn_all::<OnTitleScreen>);
}

fn setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("title_screen.png"),
            transform: Transform::from_xyz(
                SCREEN_SIZE_X / 2.,
                SCREEN_SIZE_Y / 2.,
                0.,
            ),
            ..Default::default()
        },
        OnTitleScreen,
    ));
}

fn change_scene(
    keys: Res<ButtonInput<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        game_state.set(GameState::Game)
    }
}

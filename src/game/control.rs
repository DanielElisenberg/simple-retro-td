use bevy::{
    asset::AssetServer,
    input::ButtonInput,
    prelude::{Commands, KeyCode, Query, Res, Transform, With},
    sprite::SpriteBundle,
};

use crate::{
    constants::{self, SELECTOR_BOUNDS_X_MIN, SELECTOR_BOUNDS_Y_MIN},
    game::components::Selector,
    OnGameScreen,
};

pub fn move_selector(
    input: Res<ButtonInput<KeyCode>>,
    mut selector_query: Query<&mut Transform, With<Selector>>,
) {
    let selector_transform: &mut Transform =
        &mut selector_query.get_single_mut().unwrap();
    if input.just_pressed(KeyCode::ArrowLeft)
        && selector_transform.translation.x > constants::SELECTOR_BOUNDS_X_MIN
    {
        selector_transform.translation.x -= 16.
    } else if input.just_pressed(KeyCode::ArrowRight)
        && selector_transform.translation.x < constants::SELECTOR_BOUNDS_X_MAX
    {
        selector_transform.translation.x += 16.
    } else if input.just_pressed(KeyCode::ArrowUp)
        && selector_transform.translation.y < constants::SELECTOR_BOUNDS_Y_MAX
    {
        selector_transform.translation.y += 16.
    } else if input.just_pressed(KeyCode::ArrowDown)
        && selector_transform.translation.y > constants::SELECTOR_BOUNDS_Y_MIN
    {
        selector_transform.translation.y -= 16.
    }
}

pub fn spawn_selector(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("selector.png"),
            transform: Transform::from_xyz(6., 8., 2.),
            ..Default::default()
        },
        OnGameScreen,
        Selector,
    ));
}

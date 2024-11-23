use bevy::{
    asset::{AssetServer, Assets},
    input::ButtonInput,
    prelude::{Commands, KeyCode, Query, Res, ResMut, Transform, With},
    sprite::{SpriteBundle, TextureAtlasLayout},
};

use crate::{
    constants::{self, SELECTOR_BOUNDS_X_MIN, SELECTOR_BOUNDS_Y_MIN},
    game::{
        components::{OnGameScreen, Selector, TowerType},
        towers,
    },
};

pub fn move_selector(
    mut commands: Commands,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut selector_query: Query<&mut Transform, With<Selector>>,
) {
    let selector_transform: &mut Transform =
        &mut selector_query.get_single_mut().unwrap();
    if input.just_pressed(KeyCode::ArrowLeft)
        && selector_transform.translation.x > constants::SELECTOR_BOUNDS_X_MIN
    {
        selector_transform.translation.x -= 16.;
    } else if input.just_pressed(KeyCode::ArrowRight)
        && selector_transform.translation.x < constants::SELECTOR_BOUNDS_X_MAX
    {
        selector_transform.translation.x += 16.;
    } else if input.just_pressed(KeyCode::ArrowUp)
        && selector_transform.translation.y < constants::SELECTOR_BOUNDS_Y_MAX
    {
        selector_transform.translation.y += 16.;
    } else if input.just_pressed(KeyCode::ArrowDown)
        && selector_transform.translation.y > constants::SELECTOR_BOUNDS_Y_MIN
    {
        selector_transform.translation.y -= 16.;
    } else if input.just_pressed(KeyCode::KeyI) {
        towers::spawn_tower(
            &mut commands,
            texture_atlas_layouts,
            &asset_server,
            selector_transform.clone(),
            TowerType::Ice,
        );
    } else if input.just_pressed(KeyCode::KeyC) {
        towers::spawn_tower(
            &mut commands,
            texture_atlas_layouts,
            &asset_server,
            selector_transform.clone(),
            TowerType::Cannon,
        );
    } else if input.just_pressed(KeyCode::KeyA) {
        towers::spawn_tower(
            &mut commands,
            texture_atlas_layouts,
            &asset_server,
            selector_transform.clone(),
            TowerType::Arrow,
        );
    }
}

pub fn spawn_selector(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("selector.png"),
            transform: Transform::from_xyz(
                SELECTOR_BOUNDS_X_MIN - 8.,
                SELECTOR_BOUNDS_Y_MIN - 8.,
                2.,
            ),
            ..Default::default()
        },
        OnGameScreen,
        Selector,
    ));
}

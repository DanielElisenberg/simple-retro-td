use bevy::{
    asset::{AssetServer, Assets},
    audio::{AudioBundle, PlaybackMode, PlaybackSettings, Volume},
    input::ButtonInput,
    prelude::{
        default, Commands, KeyCode, Query, Res, ResMut, Transform, With,
    },
    sprite::{SpriteBundle, TextureAtlasLayout},
};

use crate::{
    constants::{self, SELECTOR_BOUNDS_X_MIN, SELECTOR_BOUNDS_Y_MIN},
    game::{
        components::{OnGameScreen, Selector, TowerType},
        resources, towers,
    },
};
fn play_error_sound(mut commands: Commands, asset_server: &Res<AssetServer>) {
    commands.spawn(AudioBundle {
        source: asset_server.load("audio/error.mp3"),
        settings: PlaybackSettings {
            mode: PlaybackMode::Once,
            volume: Volume::new(1.),
            ..default()
        },
    });
}

pub fn move_selector(
    mut commands: Commands,
    mut block_list: ResMut<resources::BlockList>,
    mut player: ResMut<resources::Player>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: Res<AssetServer>,
    input: Res<ButtonInput<KeyCode>>,
    mut selector_query: Query<&mut Transform, With<Selector>>,
) {
    // move selector
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
    }

    // guard blocked tiles
    if (input.just_pressed(KeyCode::KeyI)
        || input.just_pressed(KeyCode::KeyC)
        || input.just_pressed(KeyCode::KeyA))
        && block_list.0.contains(&(
            selector_transform.translation.x,
            selector_transform.translation.y,
        ))
    {
        play_error_sound(commands, &asset_server);
        return;
    }

    // build towers
    if input.just_pressed(KeyCode::KeyI) {
        if player.money < 20 {
            play_error_sound(commands, &asset_server);
            return;
        }
        player.money -= 20;
        block_list.0.push((
            selector_transform.translation.x,
            selector_transform.translation.y,
        ));
        towers::spawn_tower(
            &mut commands,
            texture_atlas_layouts,
            &asset_server,
            selector_transform.clone(),
            TowerType::Ice,
        );
    } else if input.just_pressed(KeyCode::KeyC) {
        if player.money < 15 {
            play_error_sound(commands, &asset_server);
            return;
        }
        player.money -= 15;
        block_list.0.push((
            selector_transform.translation.x,
            selector_transform.translation.y,
        ));
        towers::spawn_tower(
            &mut commands,
            texture_atlas_layouts,
            &asset_server,
            selector_transform.clone(),
            TowerType::Cannon,
        );
    } else if input.just_pressed(KeyCode::KeyA) {
        if player.money < 10 {
            play_error_sound(commands, &asset_server);
            return;
        }
        player.money -= 10;
        block_list.0.push((
            selector_transform.translation.x,
            selector_transform.translation.y,
        ));
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
            texture: asset_server.load("sprites/control/selector.png"),
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

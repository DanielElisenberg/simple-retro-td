use bevy::{
    asset::AssetServer,
    prelude::{Commands, Res, Transform},
    sprite::SpriteBundle,
};

use crate::game::components::{OnGameScreen, Tower};

pub fn spawn_tower(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    position: Transform,
) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("towers/arrow_tower.png"),
            transform: position,
            ..Default::default()
        },
        Tower,
        OnGameScreen,
    ));
}

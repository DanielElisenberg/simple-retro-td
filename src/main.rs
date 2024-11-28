mod common;
mod constants;
mod game;
mod title;

use bevy::prelude::*;
use bevy_embedded_assets::EmbeddedAssetPlugin;
use bevy_kira_audio::prelude::*;
use constants::{
    SCALE, SCALED_SCREEN_SIZE_X, SCALED_SCREEN_SIZE_Y, SCREEN_SIZE_X,
    SCREEN_SIZE_Y,
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Game,
    #[default]
    Title,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins((
            EmbeddedAssetPlugin::default(),
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Simple Retro TD".into(),
                        resolution: (
                            SCALED_SCREEN_SIZE_X,
                            SCALED_SCREEN_SIZE_Y,
                        )
                            .into(),
                        resizable: true,
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        ))
        .add_plugins(AudioPlugin)
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_plugins((game::plugin, title::plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform {
            translation: Vec3::new(SCREEN_SIZE_X / 2., SCREEN_SIZE_Y / 2., 0.),
            scale: Vec3::new(1. / SCALE, 1. / SCALE, 1.),
            ..Default::default()
        },
        ..Default::default()
    });
}

mod common;
mod constants;
mod game;
mod title;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    Game,
    #[default]
    Title,
}

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
struct OnTitleScreen;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.04, 0.04, 0.04)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Simple Retro TD".into(),
                resolution:
                    (constants::SCREEN_SIZE_X, constants::SCREEN_SIZE_Y).into(),
                resizable: true,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_systems(Startup, setup)
        .init_state::<GameState>()
        .add_plugins((game::plugin, title::plugin))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle { ..default() });
}

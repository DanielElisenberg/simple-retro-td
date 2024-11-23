use bevy::{
    asset::{AssetServer, Assets},
    math::UVec2,
    prelude::{default, Commands, Query, Res, ResMut, Transform, With},
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
};

use crate::game::components::{
    AnimationIndices, AnimationTimer, OnGameScreen, Tower, TowerType,
};

pub fn spawn_tower(
    commands: &mut Commands,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    asset_server: &Res<AssetServer>,
    position: Transform,
    tower_type: TowerType,
) {
    match tower_type {
        TowerType::Arrow => {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("towers/arrow_tower.png"),
                    transform: position,
                    ..Default::default()
                },
                Tower { tower_type },
                OnGameScreen,
            ));
        }
        TowerType::Cannon => {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("towers/cannon_tower.png"),
                    transform: position,
                    ..Default::default()
                },
                Tower { tower_type },
                OnGameScreen,
            ));
        }
        TowerType::Ice => {
            let spritesheet = asset_server.load("towers/ice_tower.png");
            let layout = TextureAtlasLayout::from_grid(
                UVec2::splat(16),
                4,
                1,
                None,
                None,
            );
            let texture_atlas_layout = texture_atlas_layouts.add(layout);
            let animation_indices = AnimationIndices { first: 1, last: 3 };
            commands.spawn((
                SpriteBundle {
                    transform: position,
                    texture: spritesheet,
                    ..default()
                },
                TextureAtlas {
                    layout: texture_atlas_layout,
                    index: animation_indices.first,
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
                OnGameScreen,
                Tower {
                    tower_type: TowerType::Ice,
                },
            ));
        }
    };
}

pub fn animate_towers(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas),
        With<Tower>,
    >,
) {
    for (indices, mut timer, mut atlas) in &mut query {
        timer.0.tick(time.delta());
        if timer.0.just_finished() {
            atlas.index = if atlas.index == indices.last {
                indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}

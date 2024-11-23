use bevy::{
    asset::{AssetServer, Assets},
    math::{UVec2, Vec3},
    prelude::{default, Commands, Entity, Query, Res, ResMut, Transform, With},
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
};

use crate::game::{
    components::{
        AnimationIndices, AnimationTimer, BulletType, Enemy, OnGameScreen,
        Tower, TowerType,
    },
    projectiles,
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
                Tower {
                    tower_type,
                    reload: Timer::from_seconds(0.5, TimerMode::Repeating),
                },
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
                Tower {
                    tower_type,
                    reload: Timer::from_seconds(0.5, TimerMode::Repeating),
                },
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
                    tower_type,
                    reload: Timer::from_seconds(0.5, TimerMode::Repeating),
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

pub fn shoot_from_tower(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut towers: Query<(&mut Tower, &Transform), With<Tower>>,
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut tower, tower_transform) in towers.iter_mut() {
        tower.reload.tick(time.delta());
        if tower.reload.finished() {
            let mut smallest_distance = f32::MAX;
            let mut enemy_target: Option<Entity> = None;
            for (enemy, enemy_transform) in enemies.iter() {
                let distance_to =
                    tower_transform.translation.distance(Vec3::new(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y,
                        tower_transform.translation.z,
                    ));
                if distance_to < 32. && distance_to < smallest_distance {
                    smallest_distance = distance_to;
                    enemy_target = Some(enemy);
                }
            }
            if let Some(valid_target) = enemy_target {
                projectiles::spawn_bullet(
                    &mut commands,
                    &asset_server,
                    tower_transform,
                    valid_target,
                    match tower.tower_type {
                        TowerType::Arrow => BulletType::Arrow,
                        TowerType::Cannon => BulletType::Cannonball,
                        TowerType::Ice => BulletType::Ice,
                    },
                );
                tower.reload.reset();
            }
        }
    }
}

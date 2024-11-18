use bevy::{
    asset::{AssetServer, Assets},
    math::{UVec2, Vec2},
    prelude::{
        default, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut,
        Transform, With,
    },
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
};

use crate::game::{
    components::{
        AnimationIndices, AnimationTimer, Enemy, EnemySpawner, OnGameScreen,
    },
    levels::{get_enemy_config_for_level, LevelConfig},
};

const ENEMY_PATH: [Vec2; 11] = [
    Vec2::new(5. * 16. + 8., 9. * 16. + 8.),
    Vec2::new(8. * 16. + 8., 9. * 16. + 8.),
    Vec2::new(8. * 16. + 8., 2. * 16. + 8.),
    Vec2::new(2. * 16. + 8., 2. * 16. + 8.),
    Vec2::new(2. * 16. + 8., 4. * 16. + 8.),
    Vec2::new(5. * 16. + 8., 4. * 16. + 8.),
    Vec2::new(5. * 16. + 8., 7. * 16. + 8.),
    Vec2::new(2. * 16. + 8., 7. * 16. + 8.),
    Vec2::new(2. * 16. + 8., 9. * 16. + 8.),
    Vec2::new(3. * 16. + 8., 9. * 16. + 8.),
    Vec2::new(3. * 16. + 8., 10. * 16. + 8.),
];

pub fn move_enemy(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut enemy) in query.iter_mut() {
        let current_position = transform.translation.truncate();
        let direction = ENEMY_PATH[enemy.on_step] - current_position;
        let distance = direction.length();

        if distance < 0.5 {
            enemy.on_step += 1;
            if enemy.on_step == ENEMY_PATH.len() {
                commands.entity(entity).despawn_recursive();
            }
        } else {
            let direction = direction.normalize();
            transform.translation +=
                (direction * enemy.speed * time.delta_seconds()).extend(0.0);
        }
    }
}

pub fn ysort_enemies(mut enemy_transforms: Query<&mut Transform, With<Enemy>>) {
    for mut enemy_transform in enemy_transforms.iter_mut() {
        enemy_transform.translation.z = enemy_transform.translation.y;
    }
}

pub fn spawn_enemies_from_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawners: Query<(&mut EnemySpawner, &Transform)>,
    time: Res<Time>,
) {
    let Ok((mut spawner, transform)) = spawners.get_single_mut() else {
        return;
    };
    spawner.spawn_timer.tick(time.delta());
    spawner.level_timer.tick(time.delta());

    if spawner.level_timer.finished() {
        if spawner.spawn_counter >= 30 {
            spawner.level_timer.reset();
            spawner.current_level += 1;
            spawner.spawn_counter = 0;
            return;
        }
        if spawner.spawn_timer.finished() {
            spawn_enemy(
                &mut commands,
                &asset_server,
                texture_atlas_layouts,
                transform.clone(),
                get_enemy_config_for_level(spawner.current_level),
            );
            spawner.spawn_counter += 1;
            spawner.spawn_timer.reset();
        }
    }
}

pub fn init_enemy_spawner(commands: &mut Commands) {
    commands.spawn((
        OnGameScreen,
        Transform::from_xyz(5. * 16. + 8., 10. * 16. + 8., 2.),
        EnemySpawner {
            spawn_timer: Timer::from_seconds(0.75, TimerMode::Once),
            spawn_counter: 0,
            current_level: 0,
            level_timer: Timer::from_seconds(30., TimerMode::Once),
        },
    ));
}

pub fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    transform: Transform,
    enemy_config: LevelConfig,
) {
    let spritesheet = asset_server.load(enemy_config.mob_spritesheet);
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 1, last: 3 };
    commands.spawn((
        SpriteBundle {
            transform,
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
        Enemy {
            on_step: 0,
            health: enemy_config.mob_health,
            speed: enemy_config.mob_speed,
        },
    ));
}

pub fn animate_enemy(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlas,
    )>,
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

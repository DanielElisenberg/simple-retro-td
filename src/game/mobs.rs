use bevy::{
    asset::{AssetServer, Assets},
    math::UVec2,
    prelude::{
        default, Commands, DespawnRecursiveExt, Entity, Query, Res, ResMut,
        Transform, With,
    },
    sprite::{SpriteBundle, TextureAtlas, TextureAtlasLayout},
    time::{Time, Timer, TimerMode},
};

use crate::{
    constants::MOB_PATH,
    game::{
        components::{
            AnimationIndices, AnimationTimer, Mob, MobSpawner, OnGameScreen,
        },
        levels::{get_config_for_level, LevelConfig},
        resources::Player,
    },
};

pub fn move_mobs(
    mut commands: Commands,
    mut player: ResMut<Player>,
    mut query: Query<(Entity, &mut Transform, &mut Mob)>,
    time: Res<Time>,
) {
    for (entity, mut transform, mut mob) in query.iter_mut() {
        let current_position = transform.translation.truncate();
        let direction = MOB_PATH[mob.on_step] - current_position;
        let distance = direction.length();

        if distance < 0.5 {
            mob.on_step += 1;
            if mob.on_step == MOB_PATH.len() {
                player.life = player.life.saturating_sub(1);
                commands.entity(entity).despawn_recursive();
            }
        } else {
            let direction = direction.normalize();
            transform.translation +=
                (direction * mob.speed * time.delta_seconds()).extend(0.0);
        }
    }
}

pub fn ysort_mobs(mut mob_transforms: Query<&mut Transform, With<Mob>>) {
    for mut mob_transform in mob_transforms.iter_mut() {
        mob_transform.translation.z = mob_transform.translation.y;
    }
}

pub fn spawn_mobs_from_spawner(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut spawners: Query<(&mut MobSpawner, &Transform)>,
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
            spawn_mob(
                &mut commands,
                &asset_server,
                texture_atlas_layouts,
                transform.clone(),
                get_config_for_level(spawner.current_level),
            );
            spawner.spawn_counter += 1;
            spawner.spawn_timer.reset();
        }
    }
}

pub fn init_mob_spawner(commands: &mut Commands) {
    commands.spawn((
        OnGameScreen,
        Transform::from_xyz(5. * 16. + 8., 10. * 16. + 8., 2.),
        MobSpawner {
            spawn_timer: Timer::from_seconds(0.75, TimerMode::Once),
            spawn_counter: 0,
            current_level: 0,
            level_timer: Timer::from_seconds(20., TimerMode::Once),
        },
    ));
}

pub fn spawn_mob(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    transform: Transform,
    level_config: LevelConfig,
) {
    let spritesheet = asset_server.load(level_config.mob_spritesheet);
    let layout =
        TextureAtlasLayout::from_grid(UVec2::splat(16), 4, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    let animation_indices = AnimationIndices { first: 0, last: 3 };
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
        Mob {
            on_step: 0,
            health: level_config.mob_health,
            speed: level_config.mob_speed,
            debufs: Vec::new(),
        },
    ));
}

pub fn animate_mobs(
    time: Res<Time>,
    mut query: Query<
        (&AnimationIndices, &mut AnimationTimer, &mut TextureAtlas),
        With<Mob>,
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

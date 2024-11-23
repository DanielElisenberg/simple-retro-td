use crate::game::components::{Bullet, BulletType, Enemy, OnGameScreen};
use bevy::{
    prelude::{
        Commands, DespawnRecursiveExt, Entity, Query, Res, Transform, Without,
    },
    sprite::SpriteBundle,
    time::Time,
};

pub fn move_bullet_to_target(
    mut commands: Commands,
    mut bullets: Query<(Entity, &mut Transform, &Bullet), Without<Enemy>>,
    mut enemies: Query<(Entity, &mut Transform, &mut Enemy)>,
    time: Res<Time>,
) {
    for (bullet_e, mut bullet_transform, bullet) in bullets.iter_mut() {
        if let Ok((enemy_e, enemy_transform, mut enemy)) =
            enemies.get_mut(bullet.target)
        {
            if bullet_transform.translation.distance(
                Transform::from_xyz(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                    bullet_transform.translation.z,
                )
                .translation,
            ) < 8.
            {
                commands.entity(bullet_e).despawn();
                enemy.health = enemy.health.saturating_sub(1);
                if enemy.health == 0 {
                    commands.entity(enemy_e).despawn_recursive();
                }
            } else {
                let direction = (enemy_transform.translation
                    - bullet_transform.translation)
                    .normalize();
                bullet_transform.translation +=
                    direction * 400. * time.delta_seconds();
            }
        } else {
            commands.entity(bullet_e).despawn();
        }
    }
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<bevy::asset::AssetServer>,
    tower_transform: &Transform,
    valid_target: Entity,
    bullet_type: BulletType,
) {
    let texture = asset_server.load(match bullet_type {
        BulletType::Arrow => "projectiles/arrow_projectile.png",
        BulletType::Cannonball => "projectiles/cannonball_projectile.png",
        BulletType::Ice => "projectiles/frost_projectile.png",
    });
    commands.spawn((
        SpriteBundle {
            texture,
            transform: Transform::from_xyz(
                tower_transform.translation.x,
                tower_transform.translation.y,
                100.,
            ),
            ..Default::default()
        },
        OnGameScreen,
        Bullet {
            target: valid_target,
            bullet_type,
        },
    ));
}

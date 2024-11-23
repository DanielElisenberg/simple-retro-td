use bevy::{
    prelude::{Component, Entity},
    time::Timer,
};

#[derive(Component)]
pub struct Selector;

#[derive(Component)]
pub struct OnGameScreen;

pub enum TowerType {
    Ice,
    Cannon,
    Arrow,
}

#[derive(Component)]
pub struct Tower {
    pub tower_type: TowerType,
    pub reload: Timer,
}

#[derive(Component)]
pub struct MobSpawner {
    pub spawn_timer: Timer,
    pub spawn_counter: i32,
    pub level_timer: Timer,
    pub current_level: usize,
}

#[derive(Component, Clone, Debug)]
pub struct Mob {
    pub on_step: usize,
    pub health: u8,
    pub speed: f32,
    pub debufs: Vec<Debuf>,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Bullet {
    pub target: Entity,
    pub bullet_type: BulletType,
}

#[derive(PartialEq, Clone, Debug)]
pub enum DebufEffect {
    Frozen,
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Debuf {
    pub effect: DebufEffect,
    pub duration: Timer,
}

pub enum BulletType {
    Arrow,
    Cannonball,
    Ice,
}

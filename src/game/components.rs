use bevy::{prelude::Component, time::Timer};

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
}

#[derive(Component)]
pub struct EnemySpawner {
    pub spawn_timer: Timer,
    pub spawn_counter: i32,
    pub level_timer: Timer,
    pub current_level: usize,
}

#[derive(Component, Clone, Debug)]
pub struct Enemy {
    pub on_step: usize,
    pub health: u8,
    pub speed: f32,
}

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component)]
pub struct AnimationTimer(pub Timer);

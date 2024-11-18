use bevy::{prelude::Component, time::Timer};

#[derive(Component)]
pub struct Selector;

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct Tower {
    pub time_out: Timer,
}

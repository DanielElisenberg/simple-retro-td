use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Player {
    pub life: u8,
    pub money: u8,
}

use bevy::prelude::Resource;

#[derive(Resource)]
pub struct Player {
    pub life: u8,
    pub money: u8,
}

#[derive(Resource)]
pub struct BlockList(pub Vec<(f32, f32)>);

#[derive(Clone)]
pub struct LevelConfig {
    pub mob_spritesheet: String,
    pub mob_health: u8,
    pub mob_speed: f32,
}

pub fn get_enemy_config_for_level(level: usize) -> LevelConfig {
    let clamped_level = if level > 5 { 5 } else { level };
    Vec::from([
        LevelConfig {
            mob_spritesheet: String::from("mobs/slime.png"),
            mob_health: 3,
            mob_speed: 30.,
        },
        LevelConfig {
            mob_spritesheet: String::from("mobs/goblin.png"),
            mob_health: 5,
            mob_speed: 20.,
        },
        LevelConfig {
            mob_spritesheet: String::from("mobs/bat.png"),
            mob_health: 2,
            mob_speed: 40.,
        },
        LevelConfig {
            mob_spritesheet: String::from("mobs/slime.png"),
            mob_health: 6,
            mob_speed: 30.,
        },
        LevelConfig {
            mob_spritesheet: String::from("mobs/goblin.png"),
            mob_health: 10,
            mob_speed: 20.,
        },
        LevelConfig {
            mob_spritesheet: String::from("mobs/bat.png"),
            mob_health: 4,
            mob_speed: 40.,
        },
    ])[clamped_level]
        .clone()
}

use bevy::{
    input::ButtonInput,
    prelude::{
        Commands, Component, DespawnRecursiveExt, Entity, KeyCode, Query, Res,
        With,
    },
};

pub fn despawn_all<T: Component>(
    to_despawn: Query<Entity, With<T>>,
    mut commands: Commands,
) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}

#[allow(dead_code)]
pub fn print_entities(
    keys: Res<ButtonInput<KeyCode>>,
    entities_query: Query<Entity>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        for entity in entities_query.iter() {
            println!("Entity: {:?}", entity);
        }
    }
}

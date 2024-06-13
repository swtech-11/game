use bevy::prelude::*;

use crate::game_logic::entities::creature::{Creature, Nutrition};

#[derive(Resource)]
pub struct State(u128);

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State(0))
            .add_systems(PreUpdate, update)
            .add_systems(Update, entities);
    }
}

fn entities(query: Query<(Entity, &Nutrition, &Transform), With<Creature>>) {
    for (entity, nutrition, transform) in query.iter() {
        log::debug!(
            "Entity: {:?}, Nutrition: {:?}, Transform: {:?}",
            entity,
            nutrition,
            transform
        );
    }
}

fn update(mut state: ResMut<State>) {
    state.0 += 1;
    log::debug!("State: {}", state.0);
}

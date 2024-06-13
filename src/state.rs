use std::time::Instant;

use bevy::prelude::*;

use crate::game_logic::entities::creature::{Creature, Nutrition};

#[derive(Resource)]
pub struct State {
    counter: u128,
    time: Instant,
}

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State {
            counter: 0,
            time: Instant::now(),
        })
        .add_systems(PreUpdate, update)
        .add_systems(Update, entities);
    }
}

fn entities(query: Query<(Entity, &Nutrition, &Transform), With<Creature>>, state: Res<State>) {
    log::debug!("Time: {:?}", Instant::now().duration_since(state.time));
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
    state.counter += 1;
    log::debug!("State: {}", state.counter);
}

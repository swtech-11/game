use std::time::Instant;

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

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

fn entities(
    query: Query<(&Transform, &Velocity), With<Creature>>,
    state: Res<State>,
    time: Res<Time>,
) {
    let mut trans: &Transform = &Transform::default();
    let mut vel: &Velocity = &Velocity::default();
    for props in query.iter() {
        trans = props.0;
        vel = props.1
    }
    log::debug!(
        "{:?}, {:?}, {:?}, {:?}",
        state.counter,
        time.elapsed_seconds(),
        trans.translation.x,
        vel.linvel.x
    );
}

fn update(mut state: ResMut<State>) {
    state.counter += 1;
}

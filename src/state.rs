use bevy::prelude::*;

#[derive(Resource)]
pub struct State(u128);

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(State(0)).add_systems(PreUpdate, update);
    }
}

fn update(mut state: ResMut<State>) {
    state.0 += 1;
    log::info!("State: {}", state.0);
}

use std::{
    fs::OpenOptions,
    io::Write,
    sync::{Arc, Mutex},
};

use bevy::prelude::*;
use bevy_rapier2d::dynamics::Velocity;

use crate::game_logic::entities::creature::{Creature, Nutrition};

pub struct StatePlugin {
    pub file_path: String,
}

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        let file = Arc::new(Mutex::new(
            OpenOptions::new()
                .create(true)
                .write(true)
                .append(true)
                .open(self.file_path.clone())
                .expect("Failed to open log file"),
        ));

        app.insert_resource(StateLog {
            file: file.clone(),
            counter: 0,
        });

        app.add_systems(Startup, write_headers)
            .add_systems(PreUpdate, update_counter)
            .add_systems(Update, write_entities);
    }
}

#[derive(Resource)]
struct StateLog {
    file: Arc<Mutex<std::fs::File>>,
    counter: u128,
}

fn write_headers(state_log: ResMut<StateLog>) {
    writeln!(
        state_log.file.lock().unwrap(),
        "{}, {}, {}, {}, {}",
        "counter",
        "time",
        "pos_x",
        "velocity",
        "nutrition"
    )
    .expect("Failed to write to log file");
}

fn write_entities(
    query: Query<(&Transform, &Velocity, &Nutrition), With<Creature>>,
    state_log: Res<StateLog>,
    time: Res<Time>,
) {
    for props in query.iter() {
        let trans = props.0;
        let vel = props.1;
        let nut = props.2;

        writeln!(
            state_log.file.lock().unwrap(),
            "{}, {}, {}, {}, {}",
            state_log.counter,
            time.elapsed_seconds(),
            trans.translation.x,
            vel.linvel.x,
            nut.0
        )
        .expect("Failed to write to state log file");
    }
}

fn update_counter(mut state_log: ResMut<StateLog>) {
    state_log.counter += 1;
}

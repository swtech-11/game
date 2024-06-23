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
        let file = match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .open(&self.file_path)
        {
            Ok(file) => Arc::new(Mutex::new(file)),
            Err(e) => {
                eprintln!("Failed to open log file: {}", e);
                return;
            }
        };

        app.insert_resource(StateLog { file, counter: 0 });

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

const HEADERS: &[&str] = &[
    "counter",
    "time",
    "pos_x",
    "pos_y",
    "rot_x",
    "rot_y",
    "rot_z",
    "velocity_linvel_x",
    "velocity_linvel_y",
    "velocity_angvel",
    "nutrition",
];

fn write_headers(state_log: ResMut<StateLog>) {
    let header_line = HEADERS.join(", ");
    if let Err(e) = writeln!(state_log.file.lock().unwrap(), "{}", header_line) {
        eprintln!("Failed to write headers to log file: {}", e);
    }
}

fn write_entities(
    query: Query<(&Transform, &Velocity, &Nutrition), With<Creature>>,
    state_log: Res<StateLog>,
    time: Res<Time>,
) {
    let mut file = state_log.file.lock().unwrap();
    for (transform, velocity, nutrition) in query.iter() {
        let data = [
            state_log.counter.to_string(),
            time.elapsed_seconds().to_string(),
            transform.translation.x.to_string(),
            transform.translation.y.to_string(),
            transform.rotation.x.to_string(),
            transform.rotation.y.to_string(),
            transform.rotation.z.to_string(),
            velocity.linvel.x.to_string(),
            velocity.linvel.y.to_string(),
            velocity.angvel.to_string(),
            nutrition.0.to_string(),
        ];
        let data_line = data.join(", ");

        if let Err(e) = writeln!(file, "{}", data_line) {
            eprintln!("Failed to write entity state to log file: {}", e);
        }
    }
}

fn update_counter(mut state_log: ResMut<StateLog>) {
    state_log.counter += 1;
}

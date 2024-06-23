use bevy::prelude::*;
use std::fs;

use crate::game_logic::entities::creature::Creature;

use super::dqn::QNetwork;

const BRAIN_DIR: &str = "brain";

pub fn load_qnetwork_from_file() -> Vec<QNetwork> {
    for entry in fs::read_dir(BRAIN_DIR).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let serialized = fs::read_to_string(path).unwrap();
        let q_network: QNetwork = serde_json::from_str(&serialized).unwrap();
        return vec![q_network];
    }
    vec![]
}

pub fn save_periodically(time: Res<Time>, query: Query<&QNetwork, With<Creature>>) {
    debug!("Time: {:?}", time.elapsed_seconds() % 10.0);
    if time.elapsed_seconds() % 10.0 < 0.009 {
        debug!("Saving creature states");
        save_creature_states(query);
    }
}

pub fn save_creature_states(query: Query<&QNetwork, With<Creature>>) {
    let mut count = 0;
    for q_network in query.iter() {
        let path = format!("{}.json", count.to_string());
        fs::create_dir_all(BRAIN_DIR).unwrap();
        let file_path = format!("{}/{}", BRAIN_DIR, path.as_str());
        save_qnetwork_to_file(q_network, &file_path);
        count += 1;
    }
}

pub fn save_qnetwork_to_file(q_network: &QNetwork, path: &str) {
    let serialized = serde_json::to_string(q_network).unwrap();
    fs::write(path, serialized).unwrap();
}

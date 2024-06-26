use bevy::prelude::*;
use std::fs;

use crate::game_logic::entities::creature::Creature;

use super::generic::QNetwork;

const BRAIN_DIR: &str = "brain";

pub struct AIPersistencyPlugin;

impl Plugin for AIPersistencyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, save_periodically);
    }
}

// For now, this is okayish because is temporary and I don't know how the final creatures will look like
pub fn load_creature_brains(mut commands: Commands, creature_query: Query<Entity, With<Creature>>) {
    let dqns = load_qnetwork_from_file();

    if dqns.is_empty() {
        warn!("No brains found in the brain directory. This means you are starting with a blank slate.");
    }

    let mut count = 0;
    for entity in creature_query.iter() {
        if dqns.is_empty() {
            commands.entity(entity).insert(QNetwork::new(&[4, 24, 4]));
            continue;
        }
        commands.entity(entity).insert(dqns[count].clone());
        count += 1;
    }
}

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

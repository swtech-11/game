use super::Health;
use crate::game_logic::ai::dqn::QNetwork;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::fs;

const BRAIN_DIR: &str = "brain";

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, action)
            .add_systems(PostUpdate, save_periodically);
    }
}

#[derive(Component, Clone, Debug)]
pub struct Creature;

#[derive(Component, Debug, Clone)]
pub struct Nutrition(pub u8);

#[derive(Component, Clone, Debug)]
pub struct ActionState {
    pub current_action: CreatureAction,
}

#[derive(Clone, Debug)]
pub enum CreatureAction {
    MoveForward,
    TurnLeft,
    TurnRight,
    None,
}

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub nutrition: Nutrition,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub transform: TransformBundle,
    pub impulse: ExternalImpulse,
    pub velocity: Velocity,
    pub health: Health,
    pub damping: Damping,
    pub action_state: ActionState,
    pub dqn: QNetwork,
    pub name: Name,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        let name = Name::new("1".to_string());
        Self {
            creature: Creature,
            nutrition: Nutrition(0),
            collider: Collider::ball(0.5),
            rigid_body: RigidBody::Dynamic,
            active_events: ActiveEvents::COLLISION_EVENTS,
            transform: TransformBundle::default(),
            impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
            health: Health(100),
            damping: Damping {
                linear_damping: 0.5,
                angular_damping: 1.0,
            },
            action_state: ActionState {
                current_action: CreatureAction::None,
            },
            name: name.clone(),
            dqn: load_qnetwork_from_file(name.to_string()),
        }
    }
}

fn action(
    mut query: Query<(&mut Transform, &mut ExternalImpulse, &mut ActionState), With<Creature>>,
) {
    for (mut transform, mut impulse, mut action_state) in query.iter_mut() {
        match action_state.current_action {
            CreatureAction::MoveForward => {
                let forward = transform.rotation.mul_vec3(Vec3::X);
                impulse.impulse = forward.xy() * 1.0;
            }
            CreatureAction::TurnLeft => {
                transform.rotate(Quat::from_rotation_z(0.05));
            }
            CreatureAction::TurnRight => {
                transform.rotate(Quat::from_rotation_z(-0.05));
            }
            CreatureAction::None => {}
        }
        action_state.current_action = CreatureAction::None;
    }
}

fn load_qnetwork_from_file(name: String) -> QNetwork {
    let path = format!("{}.json", name);
    let file_path = format!("{}/{}", BRAIN_DIR, path);
    let serialized = fs::read_to_string(file_path).unwrap();
    serde_json::from_str(&serialized).unwrap()
}

fn save_periodically(time: Res<Time>, query: Query<(&QNetwork, &Name), With<Creature>>) {
    debug!("Time: {:?}", time.elapsed_seconds() % 10.0);
    if time.elapsed_seconds() % 10.0 < 0.009 {
        debug!("Saving creature states");
        save_creature_states(query);
    }
}

fn save_creature_states(query: Query<(&QNetwork, &Name), With<Creature>>) {
    for creature in query.iter() {
        let path = format!("{}.json", creature.1.to_string());
        fs::create_dir_all(BRAIN_DIR).unwrap();
        let file_path = format!("{}/{}", BRAIN_DIR, path.as_str());
        save_qnetwork_to_file(creature.0, &file_path);
    }
}

fn save_qnetwork_to_file(q_network: &QNetwork, path: &str) {
    let serialized = serde_json::to_string(q_network).unwrap();
    fs::write(path, serialized).unwrap();
}

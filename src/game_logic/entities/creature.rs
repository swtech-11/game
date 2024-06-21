use crate::game_logic::ai::dqn::QNetwork;

use super::Health;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub struct CreaturePlugin;

impl Plugin for CreaturePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, action);
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
}

impl Default for CreatureBundle {
    fn default() -> Self {
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
            dqn: QNetwork::new(&[5, 24, 24, 4]),
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

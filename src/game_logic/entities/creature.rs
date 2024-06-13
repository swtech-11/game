use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone, Debug)]
pub struct Creature;

#[derive(Component, Debug, Clone)]
pub struct Nutrition(pub u8);

#[derive(Bundle, Clone, Debug)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub nutrition: Nutrition,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub transform: TransformBundle,
    pub impulse: ExternalImpulse,
}
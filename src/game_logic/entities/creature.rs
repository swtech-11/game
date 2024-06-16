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
    pub velocity: Velocity,
}

impl Default for CreatureBundle {
    fn default() -> Self {
        Self {
            creature: Creature,
            nutrition: Nutrition(0),
            collider: Collider::cuboid(0.5, 0.5),
            rigid_body: RigidBody::Dynamic,
            active_events: ActiveEvents::COLLISION_EVENTS,
            transform: TransformBundle::default(),
            impulse: ExternalImpulse::default(),
            velocity: Velocity::default(),
        }
    }
}

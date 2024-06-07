use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Fruit;

#[derive(Bundle)]
pub struct FruitBundle {
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub velocity: Velocity,
    pub transform: TransformBundle,
    pub mass_properties: ReadMassProperties,
    pub active_events: ActiveEvents,
    pub fruit: Fruit,
}

impl Default for FruitBundle {
    fn default() -> Self {
        Self {
            collider: Collider::ball(1.0),
            rigid_body: RigidBody::Dynamic,
            velocity: Velocity::default(),
            transform: TransformBundle {
                local: Transform::from_xyz(0.0, 0.0, 0.0),
                ..Default::default()
            },
            mass_properties: ReadMassProperties::default(),
            active_events: ActiveEvents::COLLISION_EVENTS,
            fruit: Fruit,
        }
    }
}

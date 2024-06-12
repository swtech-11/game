use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component, Clone)]
pub struct Fruit;

#[derive(Bundle, Clone)]
pub struct FruitBundle {
    pub fruit: Fruit,
    pub collider: Collider,
    pub rigid_body: RigidBody,
    pub active_events: ActiveEvents,
    pub transform: TransformBundle,
}

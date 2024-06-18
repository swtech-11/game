use bevy::prelude::*;

pub mod creature;
pub mod fruit;

#[derive(Component, Debug, Clone)]
pub struct Health(pub u8);

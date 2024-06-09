use bevy::prelude::*;

#[derive(Component)]
pub struct Creature;

#[derive(Component, Debug)]
pub struct Nutrition(pub u8);

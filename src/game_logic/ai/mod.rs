use bevy::prelude::*;
use generic::QNetwork;
use rand::Rng;

use super::entities::{
    creature::{ActionState, Creature, CreatureAction},
    fruit::Fruit,
};

mod creature;
mod generic;

pub struct AIPlugin;

impl Plugin for AIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, decision)
            .add_systems(PostUpdate, save_periodically);
    }
}

const ALPHA: f32 = 0.001;
const GAMMA: f32 = 0.99;
const EPSILON: f32 = 0.1;
const BATCH_SIZE: usize = 32;

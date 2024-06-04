use bevy::prelude::*;
use creature::CreaturePlugin;
use fruit::FruitPlugin;
use physics::PhysicsPlugin;

mod creature;
mod fruit;
mod physics;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(CreaturePlugin)
            .add_plugins(FruitPlugin);
    }
}

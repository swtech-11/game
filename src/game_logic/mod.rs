use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use creature::{Creature, CreaturePlugin};
use fruit::{Fruit, FruitPlugin};
use physics::PhysicsPlugin;

mod creature;
mod fruit;
mod physics;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(CreaturePlugin)
            .add_systems(Update, eat)
            .add_plugins(FruitPlugin);
    }
}

fn eat(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    // creature_query: Query<Entity, With<Creature>>,
    // fruit_query: Query<Entity, With<Fruit>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                commands.entity(*collider1).despawn();
            }
            _ => (),
        }
    }
}

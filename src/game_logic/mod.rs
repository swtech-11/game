use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use entities::{
    creature::{Creature, Nutrition},
    fruit::Fruit,
};
use physics::PhysicsPlugin;
use wall::WallPlugin;

pub mod entities;
mod physics;
mod wall;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_systems(Update, eat)
            .add_plugins(WallPlugin);
    }
}

fn eat(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    creature_query: Query<(Entity, &Nutrition), With<Creature>>,
    fruit_query: Query<(), With<Fruit>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                let fruit;
                let creature;

                let one = creature_query.get(*collider1).ok();
                let two = creature_query.get(*collider2).ok();

                if one.is_some() && fruit_query.contains(*collider2) {
                    creature = one.unwrap();
                    fruit = collider2;
                } else if two.is_some() && fruit_query.contains(*collider1) {
                    creature = two.unwrap();
                    fruit = collider1;
                } else {
                    continue;
                }

                commands.entity(*fruit).despawn();
                commands
                    .entity(creature.0)
                    .insert(Nutrition(creature.1 .0 + 1));
            }
            _ => (),
        }
    }
}

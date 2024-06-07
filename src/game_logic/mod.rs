use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use creature::{Creature, CreaturePlugin, Nutrition};
use fruit::{Fruit, FruitBundle};
use physics::PhysicsPlugin;
use wall::WallPlugin;

use crate::rng::in_bounds_rng;

mod creature;
mod fruit;
mod physics;
mod wall;

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugin)
            .add_plugins(CreaturePlugin)
            .add_systems(Update, eat)
            .add_systems(Startup, setup)
            .add_plugins(WallPlugin);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(FruitBundle {
        transform: in_bounds_rng(),
        ..Default::default()
    });
}

fn eat(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    creature_query: Query<(Entity, &Nutrition), With<Creature>>,
    fruit_query: Query<Entity, With<Fruit>>,
) {
    for collision_event in collision_events.read() {
        let other;
        let creature;

        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                match creature_query.get(*collider1).ok() {
                    Some(c) => {
                        other = *collider2;
                        creature = c
                    }
                    _ => match creature_query.get(*collider2).ok() {
                        Some(c) => {
                            other = *collider1;
                            creature = c
                        }
                        _ => continue,
                    },
                }

                match fruit_query.get(other).ok() {
                    Some(e) => {
                        commands.entity(e).despawn();
                        commands
                            .entity(creature.0)
                            .insert(Nutrition(creature.1 .0 + 1));
                        commands.spawn(FruitBundle {
                            transform: in_bounds_rng(),
                            ..Default::default()
                        });
                    }
                    _ => continue,
                }
            }
            _ => (),
        }
    }
}

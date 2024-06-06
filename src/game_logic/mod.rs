use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use creature::{Creature, CreaturePlugin, Nutrition};
use fruit::{Fruit, FruitPlugin};
use physics::PhysicsPlugin;
use wall::WallPlugin;

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
            .add_plugins(FruitPlugin)
            .add_plugins(WallPlugin);
    }
}

fn eat(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    creature_query: Query<&Nutrition, With<Creature>>,
    fruit_query: Query<(), With<Fruit>>,
) {
    for collision_event in collision_events.read() {
        match collision_event {
            CollisionEvent::Started(collider1, collider2, _) => {
                if fruit_query.get(*collider1).is_ok() {
                    commands.entity(*collider1).despawn();

                    if let Ok(nutrition) = creature_query.get(*collider2) {
                        commands
                            .entity(*collider2)
                            .insert(Nutrition(nutrition.0 + 1));
                    }
                }
            }
            _ => (),
        }
    }
}

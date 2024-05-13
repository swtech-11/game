use bevy::prelude::*;

pub const GRID_SIZE: u8 = 100;

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Health(bool);

#[derive(Component)]
pub struct Position {
    pub x: u8,
    pub y: u8,
}

pub struct GameLogicPlugin;

impl Plugin for GameLogicPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing GameLogicPlugin");
        app.add_systems(Update, move_creature);
    }
}

pub fn spawn_creature(mut commands: Commands, position: Position) {
    commands.spawn((Creature, Health(true), position));
}

fn move_creature(mut query: Query<&mut Position, With<Creature>>) {
    for mut position in query.iter_mut() {
        if position.x <= GRID_SIZE - 1 {
            position.x += 1;
            println!("Creature moved to x: {}", position.x);
        }
        if position.y <= GRID_SIZE - 1 {
            position.y += 1;
            println!("Creature moved to y: {}", position.y);
        }
    }
}

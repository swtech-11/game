use bevy::prelude::*;

pub const GRID_SIZE: u8 = 20;

#[derive(Component)]
pub struct Creature;

#[derive(Component)]
pub struct Fruit;

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
        #[cfg(feature = "render")]
        app.add_systems(Update, move_creature);
        #[cfg(not(feature = "render"))]
        app.add_systems(Update, move_single_creature);
    }
}

#[derive(Bundle)]
pub struct CreatureBundle {
    pub creature: Creature,
    pub health: Health,
    pub position: Position,
}

impl CreatureBundle {
    pub fn new(position: Position) -> Self {
        CreatureBundle {
            creature: Creature,
            health: Health(true),
            position,
        }
    }
}

#[derive(Bundle)]
pub struct FruitBundle {
    pub fruit: Fruit,
    pub health: Health,
    pub position: Position,
}

impl FruitBundle {
    pub fn new(position: Position) -> Self {
        FruitBundle {
            fruit: Fruit,
            health: Health(true),
            position,
        }
    }
}

fn move_creature(
    mut query: Query<&mut Position, With<Creature>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for mut position in query.iter_mut() {
        if input.pressed(KeyCode::ArrowRight) && position.x <= GRID_SIZE - 1 {
            position.x += 1;
        }
        if input.pressed(KeyCode::ArrowUp) && position.y <= GRID_SIZE - 1 {
            position.y += 1;
        }
        if input.pressed(KeyCode::ArrowLeft) && position.x > 0 {
            position.x -= 1;
        }
        if input.pressed(KeyCode::ArrowDown) && position.y > 0 {
            position.y -= 1;
        }
        println!("Creature new location x: {}, y: {}", position.x, position.y);
    }
}

fn move_single_creature(mut query: Query<&mut Position, With<Creature>>) {
    for mut position in query.iter_mut() {
        use std::io::{self, Write};
        let mut input_string = String::new();
        print!("Move with wasd: ");
        io::stdout().flush().unwrap(); // Make sure the printed text appears before input
        io::stdin().read_line(&mut input_string).unwrap();
        input_string = input_string.trim().to_string();

        if input_string == "d" && position.x <= GRID_SIZE - 1 {
            position.x += 1;
        }
        if input_string == "w" && position.y <= GRID_SIZE - 1 {
            position.y += 1;
        }
        if input_string == "a" && position.x > 0 {
            position.x -= 1;
        }
        if input_string == "s" && position.y > 0 {
            position.y -= 1;
        }
        println!("Creature new location x: {}, y: {}", position.x, position.y);
    }
}

mod config;
mod custom_debug;
use bevy::prelude::*;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};
use bevy_framepace::FramepacePlugin;
use bevy_pancam::{PanCam, PanCamPlugin};
use custom_debug::DebugPlugin;

fn main() {
    App::new()
        .add_plugins((
            #[cfg(debug_assertions)]
            DefaultPlugins.set(WindowPlugin {
                primary_window: None,
                ..default()
            }),
            DebugPlugin,
            #[cfg(not(debug_assertions))]
            DefaultPlugins,
            FramepacePlugin,
            PanCamPlugin::default(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, move_creature)
        .add_systems(Update, position_sync)
        .run();
}

#[derive(Component)]
struct Creature;

#[derive(Component)]
struct Health(bool);

#[derive(Component)]
struct Position {
    x: u8,
    y: u8,
}

fn setup(
    mut commands: Commands,
    mut settings: ResMut<bevy_framepace::FramepaceSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    use bevy_framepace::Limiter;
    settings.limiter = Limiter::from_framerate(60.0);

    commands
        .spawn(Camera2dBundle::default())
        .insert(PanCam::default());

    spawn_creature(
        commands,
        Position { x: 0, y: 0 },
        meshes.add(Rectangle::default()).into(),
        materials.add(Color::PURPLE),
    );
}

fn spawn_creature(
    mut commands: Commands,
    position: Position,
    mesh: Mesh2dHandle,
    material: Handle<ColorMaterial>,
) {
    commands.spawn((
        Creature,
        Health(true),
        position,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_scale(Vec3 {
                x: (10.0),
                y: (10.0),
                z: (0.0),
            }),
            ..default()
        },
    ));
}

fn position_sync(mut query: Query<(&Position, &mut Transform), Changed<Position>>) {
    for (position, mut transform) in query.iter_mut() {
        transform.translation = Vec3::new(position.x as f32, position.y as f32, 0.0);
    }
}

fn move_creature(mut query: Query<&mut Position>) {
    for mut position in query.iter_mut() {
        position.x += 1;
        position.y += 1;
    }
}

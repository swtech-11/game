use bevy::prelude::*;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, draw_gizmos);
    }
}

fn draw_gizmos(mut commands: Commands, mut gizmos: Gizmos) {
    let arbitrary_size = 1000;
    let start = arbitrary_size as f32;
    let end = -arbitrary_size as f32;
    for i in -arbitrary_size..arbitrary_size {
        let mut color = Color::DARK_GRAY;
        if i % 5 == 0 {
            color = Color::GRAY;
        }
        gizmos.line_2d(Vec2::new(i as f32, end), Vec2::new(i as f32, start), color);

        gizmos.line_2d(Vec2::new(start, i as f32), Vec2::new(end, i as f32), color);
    }
}

use bevy::prelude::*;

use crate::game_logic::GRID_SIZE;

use super::CELL_SIZE;

pub struct RenderDebugPlugin;

impl Plugin for RenderDebugPlugin {
    fn build(&self, app: &mut App) {
        info!("Initializing RenderDebugPlugin");
        app.add_systems(Startup, config_gizmos)
            .add_systems(Update, draw_grid);
    }
}

fn config_gizmos(mut gizmo_store: ResMut<GizmoConfigStore>) {
    let (config, _) = gizmo_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 0.1;
}

fn draw_grid(mut gizmos: Gizmos) {
    let length = GRID_SIZE as f32 * CELL_SIZE;
    for anchor_1 in 0..GRID_SIZE + 1 {
        let anchor_2 = anchor_1 as f32 * CELL_SIZE;
        gizmos.line_2d(
            Vec2::new(anchor_2, 0.0),
            Vec2::new(anchor_2, length),
            Color::GRAY,
        );
        gizmos.line_2d(
            Vec2::new(0.0, anchor_2),
            Vec2::new(length, anchor_2 as f32),
            Color::GRAY,
        );
    }
}

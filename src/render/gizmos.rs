use bevy::prelude::*;
use bevy_rapier2d::render::RapierDebugRenderPlugin;

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, draw_gizmos)
            .add_plugins(RapierDebugRenderPlugin::default());
    }
}

fn setup(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<DefaultGizmoConfigGroup>();
    config.line_width = 1.0;
}

fn draw_gizmos(mut gizmos: Gizmos) {
    let arbitrary_size = 1000;
    let start = arbitrary_size as f32;
    let end = -arbitrary_size as f32;
    for i in -arbitrary_size..arbitrary_size {
        let mut color = Color::DARK_GRAY;
        if i == 0 {
            color = Color::WHITE;
        } else if i % 5 == 0 {
            color = Color::GRAY;
        }

        let j = i as f32;

        gizmos.line_2d(Vec2::new(j, end), Vec2::new(j, start), color);

        gizmos.line_2d(Vec2::new(start, j), Vec2::new(end, j), color);
    }
}

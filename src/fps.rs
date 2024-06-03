use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

pub struct FPSPlugin;

impl Plugin for FPSPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin::default())
            .add_systems(Update, update);
    }
}

fn update(time: Res<Time>) {
    let fps = 1.0 / time.delta_seconds();
    println!("FPS: {:.2}", fps);
}

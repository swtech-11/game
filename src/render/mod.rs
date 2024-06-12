use bevy::log::LogPlugin;
use bevy::prelude::*;
use bevy::time::TimePlugin;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use camera::CameraPlugin;
use controllable::control;
use gizmos::GizmosPlugin;
use persistent_window::PersistentWindowPlugin;

mod camera;
pub mod controllable;
mod gizmos;
mod persistent_window;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins
            .build()
            .disable::<TaskPoolPlugin>()
            .disable::<TypeRegistrationPlugin>()
            .disable::<FrameCountPlugin>()
            .disable::<TimePlugin>()
            .disable::<LogPlugin>();
        app.add_plugins(default_plugins)
            .add_plugins(WorldInspectorPlugin::default())
            .add_plugins(PersistentWindowPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(GizmosPlugin)
            .add_systems(Update, control);
    }
}

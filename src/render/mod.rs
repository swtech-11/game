use bevy::prelude::*;
use bevy::time::TimePlugin;
use camera::CameraPlugin;
use gizmos::GizmosPlugin;
use persistent_window::PersistentWindowPlugin;

mod camera;
mod gizmos;
mod persistent_window;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        let default_plugins = DefaultPlugins
            .build()
            .disable::<WindowPlugin>()
            .disable::<TaskPoolPlugin>()
            .disable::<TypeRegistrationPlugin>()
            .disable::<FrameCountPlugin>()
            .disable::<TimePlugin>();
        app.add_plugins(default_plugins)
            .add_plugins(PersistentWindowPlugin)
            .add_plugins(CameraPlugin)
            .add_plugins(GizmosPlugin);
    }
}

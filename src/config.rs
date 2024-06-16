use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct ConfigRes {
    pub bounds: (Vec2, Vec2),
}

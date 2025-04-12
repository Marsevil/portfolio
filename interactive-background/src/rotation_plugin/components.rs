use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Component)]
pub struct Rotation {
    pub angular_velocity: f32,
    pub angle: f32,
    /// The angle the object start with
    pub start_angle: f32,
    /// Distance from the center
    pub distance: f32,
}

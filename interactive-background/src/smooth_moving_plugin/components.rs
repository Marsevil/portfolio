use bevy::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct SmoothMove {
    pub target_transform: Transform,
}

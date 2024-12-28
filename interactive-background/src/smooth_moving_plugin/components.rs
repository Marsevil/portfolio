use bevy::prelude::*;

const DEFAULT_MAX_VELOCITY: f32 = 100.0;
const DEFAULT_ACCELERATION_STRENGTH: f32 = 20.0;
const DEFAULT_INERTIA_FACTOR: f32 = 0.90;

#[derive(Debug, Clone, Component)]
pub struct SmoothMove {
    pub max_velocity: f32,
    pub acceleration_strength: f32,
    pub inertia_factor: f32,
    pub target_transform: Transform,
    pub velocity: Vec3,
}
impl Default for SmoothMove {
    fn default() -> Self {
        Self {
            max_velocity: DEFAULT_MAX_VELOCITY,
            acceleration_strength: DEFAULT_ACCELERATION_STRENGTH,
            inertia_factor: DEFAULT_INERTIA_FACTOR,
            target_transform: Default::default(),
            velocity: Vec3::ZERO,
        }
    }
}
impl SmoothMove {
    pub fn with_target_transform(self, target_transform: Transform) -> Self {
        Self {
            target_transform,
            ..self
        }
    }
}
impl SmoothMove {
    pub fn reset(&mut self) {
        self.velocity = Vec3::ZERO;
    }
}

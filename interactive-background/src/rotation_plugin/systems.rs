use bevy::prelude::*;
use core::f32::consts::PI;

use crate::interaction_plugin::resources::UserData;
use crate::smooth_moving_plugin::SmoothMove;

use super::Rotation;

pub fn update_angles(mut _commands: Commands, mut query: Query<&mut Rotation>, time: Res<Time>) {
    let delta = time.delta();
    for mut rotation in query.iter_mut() {
        rotation.angle += rotation.angular_velocity * delta.as_secs_f32();

        if rotation.angle > 2.0 * PI {
            rotation.angle -= 2.0 * PI;
        }
    }
}

pub fn update_transforms(
    mut _commands: Commands,
    user_data: Res<UserData>,
    mut query: Query<(Entity, &Rotation, &mut SmoothMove)>,
) {
    for (entity, rotation, mut smooth) in query.iter_mut() {
        if user_data
            .selected_ent
            .is_some_and(|selected_ent| selected_ent == entity)
        {
            continue;
        }

        let angle = rotation.angle + rotation.start_angle;
        let new_transform =
            Transform::from_rotation(Quat::from_euler(EulerRot::YZX, angle, 0.0, 0.0))
                * Transform::from_xyz(0.0, 0.0, rotation.distance);

        smooth.target_transform = new_transform;
    }
}

pub struct RotationPlugin;
impl Plugin for RotationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_angles, update_transforms));
    }
}

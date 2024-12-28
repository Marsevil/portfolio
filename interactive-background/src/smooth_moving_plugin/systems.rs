use bevy::{math::NormedVectorSpace, prelude::*};

use super::components::SmoothMove;
use crate::interaction_plugin::resources::UserData;

/// Moving speed of the entity
pub const SPEED: f32 = 100.0; // m/s
/// Distance at which the maximum speed is reached
pub const DISTANCE_THRESH: f32 = 50.0; // m
/// Minimal distance to trigger an animation
pub const MIN_DISTANCE: f32 = 0.01;

pub fn move_objects(
    _commands: Commands,
    mut objects: Query<(Entity, &SmoothMove, &GlobalTransform, &mut Transform)>,
    time: Res<Time>,
    user: Res<UserData>,
) {
    for object in objects.iter_mut() {
        let (object_id, move_data, global_obj_transform, mut obj_transform) = object;

        if user
            .selected_ent
            .is_some_and(|selected_ent| selected_ent == object_id)
        {
            continue;
        }

        let target_transform = move_data.target_transform;
        let vec_diff = target_transform.translation - global_obj_transform.translation();
        let distance = vec_diff.norm();

        if distance <= MIN_DISTANCE {
            obj_transform.translation += vec_diff;
            obj_transform.rotation = target_transform.rotation;
            continue;
        }

        let dir = vec_diff.normalize();
        let speed = if distance > DISTANCE_THRESH {
            SPEED
        } else {
            let coef = f32::log10(distance / DISTANCE_THRESH + 0.1) + 1.0;
            SPEED * coef
        };

        obj_transform.translation += dir * speed * time.delta().as_secs_f32();
        obj_transform.rotation = target_transform.rotation;
    }
}

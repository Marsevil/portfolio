use bevy::prelude::*;

use super::components::SmoothMove;
use crate::interaction_plugin::resources::UserData;

pub fn move_objects(
    _commands: Commands,
    mut objects: Query<(Entity, &mut SmoothMove, &GlobalTransform, &mut Transform)>,
    time: Res<Time>,
    user: Res<UserData>,
) {
    for object in objects.iter_mut() {
        let (object_id, mut move_data, global_obj_transform, mut obj_transform) = object;

        if user
            .selected_ent
            .is_some_and(|selected_ent| selected_ent == object_id)
        {
            continue;
        }

        let target_transform = move_data.target_transform;
        let vec_diff = target_transform.translation - global_obj_transform.translation();

        let inertia = move_data.inertia_factor * move_data.velocity;
        let dir = vec_diff.clamp_length_max(1.0);
        let acceleration = dir * move_data.acceleration_strength;
        let velocity = (inertia + acceleration).clamp_length_max(move_data.max_velocity);
        move_data.velocity = velocity;

        obj_transform.translation += velocity * time.delta_secs();
        obj_transform.rotation = target_transform.rotation;
    }
}

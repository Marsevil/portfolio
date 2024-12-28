use core::f32;

use bevy::input::mouse::MouseButton;
use bevy::math::bounding::{BoundingVolume, RayCast3d};
use bevy::math::prelude::InfinitePlane3d;
use bevy::prelude::*;

use crate::smooth_moving_plugin::SmoothMove;

use super::resources::{CastRayEvent, UnlockEntity, UserData};
use super::Aabb;

fn cast_ray(
    trigger: Trigger<CastRayEvent>,
    mut user_data: ResMut<UserData>,
    q_objs: Query<(Entity, &GlobalTransform, &Aabb)>,
) {
    let ray = RayCast3d::from_ray(trigger.event().ray, f32::INFINITY);

    let mut res: Option<Entity> = None;
    for obj in q_objs.iter() {
        let (entity, transform, aabb) = obj;
        let (_, rotation, translation) = transform.to_scale_rotation_translation();
        let aabb_glob = aabb.aabb.transformed_by(translation, rotation);
        res = ray.aabb_intersection_at(&aabb_glob).map(|_| entity);

        if res.is_some() {
            break;
        }
    }

    user_data.selected_ent = res;
}

fn handle_mouse_click(
    mut cmd: Commands,
    mouse_btn: Res<ButtonInput<MouseButton>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    if mouse_btn.just_released(MouseButton::Left) {
        cmd.trigger(UnlockEntity::ClickReleased);
        return;
    }
    if !mouse_btn.just_pressed(MouseButton::Left) {
        return;
    }

    let window = q_window.single();
    let (camera, camera_transform) = q_camera.single();

    let Some(ray) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world(camera_transform, pos).ok())
    else {
        return;
    };

    cmd.trigger(CastRayEvent { ray });
}

fn move_on_mouse(
    mut cmd: Commands,
    user_data: Res<UserData>,
    mut q_objs: Query<(&mut Transform, &GlobalTransform), With<Aabb>>,
    q_window: Query<&Window>,
    q_camera: Query<(&Camera, &GlobalTransform)>,
) {
    let Some(selected_ent) = user_data.selected_ent else {
        return;
    };

    // Get cursor position in space
    let window = q_window.single();
    let (camera, camera_global) = q_camera.single();
    let Some(pointer_ray) = window
        .cursor_position()
        .and_then(|pos| camera.viewport_to_world(camera_global, pos).ok())
    else {
        cmd.trigger(UnlockEntity::MouseExited);
        return;
    };

    let (mut obj_transform, obj_global) = q_objs.get_mut(selected_ent).unwrap();

    // Move the object in his plane
    let moving_plane = (
        InfinitePlane3d::new(camera_global.right().cross(camera_global.up().into())),
        obj_global.translation(),
    );
    let target_pos_glob = {
        let Some(intersection_dist) = pointer_ray.intersect_plane(moving_plane.1, moving_plane.0)
        else {
            cmd.trigger(UnlockEntity::MovingPlaneIntersectionNotFound);
            return;
        };
        pointer_ray.get_point(intersection_dist)
    };
    let pos_diff = target_pos_glob - obj_global.translation();
    obj_transform.translation += pos_diff;
}

fn unlock_entity(
    _trigger: Trigger<UnlockEntity>,
    mut q_entities: Query<&mut SmoothMove>,
    mut user_data: ResMut<UserData>,
) {
    let Some(locked_entity) = user_data.selected_ent else {
        return;
    };

    if let Ok(mut move_data) = q_entities.get_mut(locked_entity) {
        move_data.reset();
    }
    user_data.selected_ent = None;
}

pub struct InteractionPlugin;
impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_mouse_click)
            .add_systems(Update, move_on_mouse)
            .insert_resource(UserData::default())
            .add_observer(cast_ray)
            .add_observer(unlock_entity);
    }
}

use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use core::f32::consts::PI;

use crate::interaction_plugin::Aabb;
use crate::rotation_plugin::Rotation;

const CUBE_PATH: &str = "models/Portal Companion Cube.glb#Scene0";

#[derive(Bundle)]
pub struct RotationBundle {
    pub scene: SceneBundle,
    pub rotation: Rotation,
    pub aabb: Aabb,
}

fn scene_init(mut commands: Commands, asset_server: Res<AssetServer>) {
    let cube_model = asset_server.load(CUBE_PATH);

    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 5.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            shadows_enabled: false,
            ..default()
        },
        ..default()
    });

    commands.spawn(RotationBundle {
        scene: SceneBundle {
            scene: cube_model.clone(),
            ..default()
        },
        rotation: Rotation {
            angle: 0.0,
            angular_velocity: -1.0,
            start_angle: 0.0,
            distance: 0.0,
        },
        aabb: Aabb {
            aabb: Aabb3d {
                min: Vec3::new(-2.0, -2.0, -2.0).into(),
                max: Vec3::new(2.0, 2.0, 2.0).into(),
            },
        },
    });

    commands.spawn(RotationBundle {
        rotation: Rotation {
            angle: 0.0,
            angular_velocity: 1.0,
            distance: 10.0,
            start_angle: 0.0,
        },
        scene: SceneBundle {
            scene: cube_model.clone(),
            ..default()
        },
        aabb: Aabb {
            aabb: Aabb3d {
                min: Vec3::new(-2.0, -2.0, -2.0).into(),
                max: Vec3::new(2.0, 2.0, 2.0).into(),
            },
        },
    });

    commands.spawn(RotationBundle {
        rotation: Rotation {
            angle: 0.0,
            angular_velocity: 1.0,
            distance: 10.0,
            start_angle: PI,
        },
        scene: SceneBundle {
            scene: cube_model.clone(),
            ..default()
        },
        aabb: Aabb {
            aabb: Aabb3d {
                min: Vec3::new(-2.0, -2.0, -2.0).into(),
                max: Vec3::new(2.0, 2.0, 2.0).into(),
            },
        },
    });
}

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, scene_init);
    }
}

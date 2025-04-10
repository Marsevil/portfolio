use bevy::asset::RenderAssetUsages;
use bevy::core_pipeline::Skybox;
use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};
use core::f32::consts::PI;

use crate::interaction_plugin::Aabb;
use crate::rotation_plugin::Rotation;
use crate::skybox_plugin::SkyboxImage;
use crate::smooth_moving_plugin::SmoothMove;

const NB_CUBE_FACES: u8 = 6;
const CUBE_PATH: &str = "models/Portal Companion Cube.glb#Scene0";

#[derive(Bundle)]
pub struct EntityBundle {
    pub transform: Transform,
    pub scene: SceneRoot,
    pub rotation: Rotation,
    pub aabb: Aabb,
    pub smooth: SmoothMove,
}

fn init_entities(mut commands: Commands, assets: Res<AssetServer>) {
    let cube_model = assets.load(CUBE_PATH);

    let new_cube = |start_angle: f32, distance: f32| -> EntityBundle {
        let rotation = Quat::from_rotation_y(start_angle);
        let transform = Transform::default()
            .with_rotation(rotation)
            .with_translation(rotation * Vec3::ZERO.with_z(distance));
        EntityBundle {
            scene: SceneRoot(cube_model.clone()),
            transform,
            rotation: Rotation {
                angle: 0.0,
                angular_velocity: -1.0,
                start_angle,
                distance,
            },
            aabb: Aabb {
                aabb: Aabb3d {
                    min: Vec3::new(-1.0, -1.0, -1.0).into(),
                    max: Vec3::new(1.0, 1.0, 1.0).into(),
                },
            },
            smooth: SmoothMove::default().with_target_transform(transform),
        }
    };

    commands.spawn(new_cube(0.0, 0.0));
    commands.spawn(new_cube(PI / 2.0, 10.0));
    commands.spawn(new_cube(-PI / 2.0, 10.0));
}

fn init_light(mut cmd: Commands) {
    cmd.spawn(DirectionalLight {
        shadows_enabled: false,
        ..Default::default()
    });
}

fn init_camera(mut cmd: Commands, mut images: ResMut<Assets<Image>>) {
    let size = Extent3d {
        width: 1024,
        height: 6 * 1024,
        ..default()
    };
    let mut image = Image::new_fill(
        size,
        TextureDimension::D2,
        &[0, 0, 0, 255],
        TextureFormat::Rgba8Unorm,
        RenderAssetUsages::default(),
    );
    image.texture_descriptor.usage =
        TextureUsages::COPY_DST | TextureUsages::TEXTURE_BINDING | TextureUsages::RENDER_ATTACHMENT;
    image.reinterpret_stacked_2d_as_array(NB_CUBE_FACES as u32);
    image.texture_view_descriptor = Some(TextureViewDescriptor {
        dimension: Some(TextureViewDimension::Cube),
        ..default()
    });

    let image_handle = images.add(image);

    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        Transform::from_xyz(0.0, 5.0, -15.0).looking_at(Vec3::ZERO, Vec3::Y),
        Skybox {
            image: image_handle.clone(),
            brightness: 1000.0,
            ..Default::default()
        },
    ));

    cmd.insert_resource(SkyboxImage(image_handle));
}

pub struct ScenePlugin;
impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (init_camera, init_light, init_entities));
    }
}

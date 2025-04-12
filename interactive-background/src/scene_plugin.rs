use core::f32;

use bevy::asset::RenderAssetUsages;
use bevy::core_pipeline::Skybox;
use bevy::math::bounding::Aabb3d;
use bevy::prelude::*;
use bevy::render::render_resource::{
    Extent3d, TextureDimension, TextureFormat, TextureUsages, TextureViewDescriptor,
    TextureViewDimension,
};

use crate::interaction_plugin::Aabb;
use crate::rotation_plugin::{transform_from_angle, Rotation};
use crate::skybox_plugin::SkyboxImage;
use crate::smooth_moving_plugin::SmoothMove;

const NB_CUBE_FACES: u8 = 6;
const ENTITIES_PATHS: [&str; 7] = [
    "models/Portal Companion Cube.glb#Scene0",
    "models/Guitar Amp.glb#Scene0",
    "models/Electric guitar.glb#Scene0",
    "models/Mushroom.glb#Scene0",
    "models/Oculus Controller.glb#Scene0",
    "models/Tree.glb#Scene0",
    "models/Videogame Controller.glb#Scene0",
];

#[derive(Bundle)]
pub struct EntityBundle {
    pub transform: Transform,
    pub scene: SceneRoot,
    pub rotation: Rotation,
    pub aabb: Aabb,
    pub smooth: SmoothMove,
}

fn new_entity(model: Handle<Scene>, angle: f32, distance: f32) -> EntityBundle {
    let transform = transform_from_angle(angle, distance);
    EntityBundle {
        scene: SceneRoot(model),
        transform,
        rotation: Rotation {
            angle: 0.0,
            angular_velocity: -1.0,
            start_angle: angle,
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
}
fn init_entities(mut commands: Commands, assets: Res<AssetServer>) {
    let angle_gap = 2.0 * core::f32::consts::PI / (ENTITIES_PATHS.len() as f32);
    for (model_idx, model_path) in ENTITIES_PATHS.iter().enumerate() {
        let model = assets.load(*model_path);
        let angle = angle_gap * (model_idx as f32);
        let scene = new_entity(model, angle, 10.0);
        commands.spawn(scene);
    }
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
        Transform::from_xyz(0.0, 5.0, -20.0).looking_at(Vec3::ZERO, Vec3::Y),
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

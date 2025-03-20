mod interaction_plugin;
mod rotation_plugin;
mod scene_plugin;
mod skybox_plugin;
mod smooth_moving_plugin;

use bevy::prelude::*;

use crate::interaction_plugin::InteractionPlugin;
use crate::rotation_plugin::RotationPlugin;
use crate::scene_plugin::ScenePlugin;
use crate::skybox_plugin::SkyboxPlugin;
use crate::smooth_moving_plugin::SmoothMovingPlugin;

fn main() {
    App::new()
        .add_plugins((
            ScenePlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    canvas: Some("#interactive-background-renderer".into()),
                    fit_canvas_to_parent: true,
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            RotationPlugin,
            InteractionPlugin,
            SmoothMovingPlugin,
            SkyboxPlugin,
        ))
        .run();
}

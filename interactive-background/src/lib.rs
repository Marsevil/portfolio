mod interaction_plugin;
mod loading_plugin;
mod rotation_plugin;
mod scene_plugin;
mod skybox_plugin;
mod smooth_moving_plugin;

use bevy::prelude::*;
use js_sys::Function as JsFunc;
use std::panic;
use wasm_bindgen::prelude::*;

use crate::interaction_plugin::InteractionPlugin;
use crate::loading_plugin::{LoadingJustFinished, LoadingPlugin};
use crate::rotation_plugin::RotationPlugin;
use crate::scene_plugin::ScenePlugin;
use crate::skybox_plugin::SkyboxPlugin;
use crate::smooth_moving_plugin::SmoothMovingPlugin;

pub struct JsInterop {
    loaded_cb: Option<JsFunc>,
}

fn loading_finished(_event: Trigger<LoadingJustFinished>, interop: NonSend<JsInterop>) {
    let Some(loaded_cb) = interop.loaded_cb.as_ref() else {
        info!("Loading finished, callback not set");
        return;
    };

    let js_ctx = JsValue::null();
    loaded_cb.call0(&js_ctx).expect("JS function call failed");
}

#[wasm_bindgen(typescript_custom_section)]
const RUN_FUNCTION_DEF: &str = r#"
export function run(onLoadingFinished: () => void)
"#;
#[wasm_bindgen(skip_typescript)]
pub fn run(on_loading_finished: &JsFunc) {
    panic::set_hook(Box::new(console_error_panic_hook::hook));

    let loaded_cb = if on_loading_finished.is_function() {
        Some(on_loading_finished.clone())
    } else {
        None
    };
    let interop = JsInterop { loaded_cb };

    App::new()
        .insert_non_send_resource(interop)
        .add_observer(loading_finished)
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
            LoadingPlugin,
            RotationPlugin,
            InteractionPlugin,
            SmoothMovingPlugin,
            SkyboxPlugin,
        ))
        .run();
}

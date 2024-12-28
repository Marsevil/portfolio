use bevy::app::{App, Plugin, Update};

mod components;
mod systems;

pub use components::SmoothMove;

pub struct SmoothMovingPlugin;
impl Plugin for SmoothMovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::move_objects);
    }
}

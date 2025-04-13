use bevy::prelude::*;

#[derive(Resource)]
pub struct LoadingState {
    pub loading_assets: Vec<UntypedHandle>,
    confirmation_frames_count: u8,
    just_finished: bool,
}
impl Default for LoadingState {
    fn default() -> Self {
        Self {
            loading_assets: Vec::new(),
            confirmation_frames_count: 0,
            just_finished: true,
        }
    }
}
impl LoadingState {
    pub const CONFIRMATION_FRAMES_TARGET: u8 = 30;

    /// Return if the loading is finished
    pub fn finished(&self) -> bool {
        self.loading_assets.is_empty()
            & (self.confirmation_frames_count == Self::CONFIRMATION_FRAMES_TARGET)
    }

    /// Returns if the loading has finished last frame
    pub fn just_finished(&self) -> bool {
        self.finished() & self.just_finished
    }
}

#[derive(Event)]
pub struct LoadingJustFinished;

fn loading_just_finished(
    _event: Trigger<LoadingJustFinished>,
    mut loading_state: ResMut<LoadingState>,
) {
    loading_state.just_finished = false;
}

fn update_loading_assets(mut loading_state: ResMut<LoadingState>, asset_server: Res<AssetServer>) {
    if loading_state.finished() {
        return;
    }
    if loading_state.loading_assets.is_empty() {
        loading_state.confirmation_frames_count += 1;
        return;
    }

    loading_state.confirmation_frames_count = 0;

    loading_state.loading_assets.retain(|asset| {
        asset_server
            .get_load_states(asset)
            .is_some_and(|state| state.2.is_loaded())
    });
}

fn check_loading_state(mut cmd: Commands, loading_state: Res<LoadingState>) {
    if loading_state.just_finished() {
        cmd.trigger(LoadingJustFinished);
    }
}

pub struct LoadingPlugin;
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (update_loading_assets, check_loading_state))
            .add_observer(loading_just_finished)
            .insert_resource(LoadingState::default());
    }
}

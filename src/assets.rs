use crate::prelude::*;

pub struct AssetsPlugin {
    pub next_state: AppState,
}

impl AssetsPlugin {
    pub fn continue_to(next_state: AppState) -> Self {
        Self { next_state }
    }
}

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_exit_system(AppState::AssetLoading, despawn_all_with::<LoadingMenu>)
            .insert_resource(NextState(self.next_state));
    }
}

#[derive(Component)]
pub struct LoadingMenu;

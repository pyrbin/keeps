#![feature(map_first_last)]
mod assets;
mod camera;
mod debug;
mod dx;
mod grid;
mod pathfinding;
pub mod prelude;
mod state;
mod utils;
mod window;

use bevy_embedded_assets::EmbeddedAssetPlugin;
pub use prelude::*;

pub const BOARD_WIDTH: i32 = 11;
pub const BOARD_CELL_SIZE: i32 = 1;
pub const UNIT_BOARD_HEIGHT: i32 = 15;
pub const KEEP_BOARD_HEIGHT: i32 = 5;

pub fn setup_app(app: &mut App) -> &mut App {
    app.add_plugin(WindowPlugin);
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });

    app.add_loopless_state(AppState::AssetLoading);

    #[cfg(debug_assertions)]
    app.add_plugin(dx::DiagnosticsPlugin);

    app.add_plugin(AssetsPlugin::continue_to(AppState::InGame));
    app.add_plugin(DebugPlugin);
    app.add_plugin(CameraPlugin);
    app.add_plugin(GridPlugin);
    app.add_plugin(PathfindingPlugin);
    app
}

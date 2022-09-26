#![feature(map_first_last)]

mod assets;
mod camera;
mod debug;
mod grid;
mod pathfinding;
pub mod prelude;
mod state;
mod utils;
mod window;

#[cfg(feature = "dev")]
mod dx;

use bevy_embedded_assets::EmbeddedAssetPlugin;
pub use prelude::*;

pub fn setup_app(app: &mut App) -> &mut App {
    app.add_plugin(WindowPlugin);
    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });

    #[cfg(feature = "dev")]
    {
        app.add_plugin(dx::DiagnosticsPlugin);
        app.register_inspectable::<Coord>();
        app.register_inspectable::<Cost>();
        app.register_inspectable::<FlowDirection>();
        log::info!("Loaded diagnostics & debugging features.");
    }

    app.add_loopless_state(AppState::AssetLoading);
    app.add_plugin(AssetsPlugin::continue_to(AppState::InGame));
    app.add_plugin(DebugPlugin);
    app.add_plugin(CameraPlugin);
    app.add_plugin(GridPlugin);
    app.add_plugin(PathfindingPlugin);
    app
}

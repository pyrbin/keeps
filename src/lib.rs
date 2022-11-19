#![feature(let_chains)]

mod assets;
mod camera;
mod debug;
mod grid;
mod pathfinding;
pub mod prelude;
mod state;
mod unit;
mod utils;
mod window;

#[cfg(feature = "dev")]
mod dx;

// TODO just for testing
mod playground;

use bevy_embedded_assets::EmbeddedAssetPlugin;
use playground::PlaygroundPlugin;
pub use prelude::*;

pub fn setup_app(app: &mut App) -> &mut App {
    app.add_plugin(WindowPlugin);
    app.add_plugins(
        DefaultPlugins
            .build()
            .add_before::<AssetPlugin, EmbeddedAssetPlugin>(EmbeddedAssetPlugin),
    );

    #[cfg(feature = "dev")]
    {
        app.add_plugin(dx::DiagnosticsPlugin);
        app.register_inspectable::<Coord>();
        app.register_inspectable::<Cost>();
        log::info!("Loaded diagnostics & debugging features.");
    }

    app.add_loopless_state(AppState::AssetLoading);
    app.add_plugin(AssetsPlugin::continue_to(AppState::InGame));
    app.add_plugin(DebugPlugin);
    app.add_plugin(CameraPlugin);
    app.add_plugin(GridPlugin);
    app.add_plugin(PathfindingPlugin);
    app.add_plugin(UnitPlugin);
    app.add_plugin(PlaygroundPlugin);
    app
}

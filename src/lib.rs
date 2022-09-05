mod assets;
mod board;
mod camera;
mod debug;
mod dx;
mod grid;
mod state;
mod utils;

pub mod prelude {
    pub use crate::assets::*;
    pub use crate::board::*;
    pub use crate::camera::*;
    pub use crate::debug::*;
    pub use crate::grid::*;
    pub use crate::state::*;
    pub use crate::utils::*;

    pub use bevy::math::Vec3Swizzles;
    pub use bevy::prelude::*;
    pub use bevy::render::texture::ImageSettings;
    pub use bevy::winit::WinitSettings;
    pub use bevy_prototype_debug_lines::DebugLines;
    pub use iyes_loopless::prelude::*;
}

use bevy::window::{PresentMode, WindowMode};
use bevy_embedded_assets::EmbeddedAssetPlugin;
pub use prelude::*;

pub const BOARD_WIDTH: i32 = 10;
pub const BOARD_CELL_SIZE: i32 = 1;
pub const UNIT_BOARD_HEIGHT: i32 = 15;
pub const KEEP_BOARD_HEIGHT: i32 = 5;

pub fn setup_app(app: &mut App) -> &mut App {
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::hex("171717").unwrap()))
        .insert_resource(WindowDescriptor {
            title: "keeps".to_string(),
            width: 1280.0,
            height: 720.0,
            position: WindowPosition::Automatic,
            scale_factor_override: Some(1.0),
            present_mode: PresentMode::AutoVsync,
            resizable: true,
            decorations: true,
            cursor_locked: false,
            cursor_visible: true,
            mode: WindowMode::Windowed,
            transparent: false,
            canvas: Some("#bevy".to_string()),
            fit_canvas_to_parent: true,
            ..Default::default()
        });

    app.add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    });

    app.add_loopless_state(AppState::AssetLoading);

    #[cfg(debug_assertions)]
    app.add_plugin(dx::DiagnosticsPlugin);

    app.add_plugin(AssetsPlugin::continue_to(AppState::WorldGen));
    app.add_plugin(DebugPlugin);
    app.add_plugin(CameraPlugin);
    app.add_plugin(GridPlugin::with_cell_size(BOARD_CELL_SIZE));
    app.add_plugin(BoardPlugin::with_settings(BoardSettings {
        unit_board: (BOARD_WIDTH, UNIT_BOARD_HEIGHT),
        keep_board: (BOARD_WIDTH, KEEP_BOARD_HEIGHT),
        offset: Vec3::new(
            -(BOARD_WIDTH * BOARD_CELL_SIZE) as f32 / 2.0,
            0.0,
            -(UNIT_BOARD_HEIGHT * BOARD_CELL_SIZE) as f32,
        ),
    }));
    app
}

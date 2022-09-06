use crate::prelude::*;
use bevy::window::{PresentMode, WindowMode};

pub struct WindowPlugin;

impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Msaa { samples: 4 })
            .insert_resource(ClearColor(Color::hex("171717").unwrap()))
            .insert_resource(WindowDescriptor {
                title: "coline".to_string(),
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
    }
}

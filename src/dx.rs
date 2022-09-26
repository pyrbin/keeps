use crate::prelude::*;
use bevy::diagnostic::Diagnostics;

pub struct DiagnosticsPlugin;

impl Plugin for DiagnosticsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_plugin(bevy::diagnostic::DiagnosticsPlugin)
            .add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin)
            .add_system(egui_display_diagnostics)
            .add_plugin(WorldInspectorPlugin::new());
    }
}

pub fn diagnostic_ui(ui: &mut egui::Ui, diagnostics: &Diagnostics, app_state: &AppState) {
    egui::Grid::new("frame time diagnostics").show(ui, |ui| {
        for diagnostic in diagnostics.iter() {
            ui.label(diagnostic.name.as_ref());
            if let Some(average) = diagnostic.average() {
                ui.label(format!("{:.2}", average));
            }
            ui.end_row();
        }

        ui.label("app_state");
        ui.label(format!("{:?}", app_state));
        ui.end_row();
    });
}

pub fn egui_display_diagnostics(
    mut egui_context: ResMut<EguiContext>,
    diagnostics: Res<Diagnostics>,
    app_state: Res<CurrentState<AppState>>,
) {
    egui::Window::new("Diagnostics")
        .min_width(0.0)
        .default_width(1.0)
        .show(egui_context.ctx_mut(), |ui| {
            diagnostic_ui(ui, &diagnostics, &app_state.0);
        });
}

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, setup_camera);
    }
}

#[derive(Component, Debug)]
pub struct MainCamera;

fn setup_camera(mut cmds: Commands) {
    cmds.spawn((Camera3dBundle::default(), MainCamera));
}

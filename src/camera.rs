use smooth_bevy_cameras::{
    controllers::orbit::{OrbitCameraBundle, OrbitCameraController, OrbitCameraPlugin},
    LookTransformPlugin,
};

use crate::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(LookTransformPlugin);
        app.add_plugin(OrbitCameraPlugin::default());
        app.add_enter_system(AppState::InGame, setup_camera);
    }
}

#[derive(Component, Debug)]
pub struct MainCamera;

fn setup_camera(mut commands: Commands) {
    commands
        .spawn_bundle(Camera3dBundle::default())
        .insert_bundle(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(0., 15., 0.),
            Vec3::new(0., 0., 0.),
        ))
        .insert(MainCamera);
}

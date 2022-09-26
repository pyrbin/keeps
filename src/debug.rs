use crate::prelude::*;
use bevy::pbr::wireframe::WireframePlugin;
use bevy_prototype_debug_lines::{DebugLines, DebugLinesPlugin};
use std::f32::consts::{FRAC_PI_2, PI};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(DebugLinesPlugin::with_depth_test(true));

        #[cfg(debug_assertions)]
        app.add_plugin(WireframePlugin);

        #[cfg(debug_assertions)]
        app.add_system_set(
            ConditionSet::new()
                .run_in_state(AppState::InGame)
                .with_system(debug_origin_axis)
                .into(),
        );
    }
}

fn debug_origin_axis(mut lines: ResMut<DebugLines>) {
    const AXIS_LENGTH: f32 = 50.0;
    lines.line_colored(Vec3::ZERO, Vec3::X * AXIS_LENGTH, 0.0, Color::RED);
    lines.line_colored(Vec3::ZERO, Vec3::NEG_X * AXIS_LENGTH, 0.0, Color::RED);

    lines.line_colored(Vec3::ZERO, Vec3::Y * AXIS_LENGTH, 0.0, Color::GREEN);
    lines.line_colored(Vec3::ZERO, Vec3::NEG_Y * AXIS_LENGTH, 0.0, Color::GREEN);

    lines.line_colored(Vec3::ZERO, Vec3::Z * AXIS_LENGTH, 0.0, Color::BLUE);
    lines.line_colored(Vec3::ZERO, Vec3::NEG_Z * AXIS_LENGTH, 0.0, Color::BLUE);
}

pub trait DebugLinesExt {
    fn circle(&mut self, origin: Vec3, rot: Quat, radius: f32, duration: f32, color: Color);
    fn square(&mut self, origin: Vec3, size: f32, duration: f32, color: Color);
}

impl DebugLinesExt for DebugLines {
    fn circle(&mut self, origin: Vec3, rot: Quat, radius: f32, duration: f32, color: Color) {
        add_circle(self, origin, rot, radius, duration, color);
    }
    fn square(&mut self, origin: Vec3, size: f32, duration: f32, color: Color) {
        add_square(self, origin, size, duration, color);
    }
}

fn add_square(lines: &mut DebugLines, origin: Vec3, size: f32, duration: f32, color: Color) {
    let half_size = size / 2.0;
    let p1 = origin + Vec3::new(-half_size, 0.0, -half_size);
    let p2 = origin + Vec3::new(half_size, 0.0, -half_size);
    let p3 = origin + Vec3::new(half_size, 0.0, half_size);
    let p4 = origin + Vec3::new(-half_size, 0.0, half_size);
    lines.line_colored(p1, p2, duration, color);
    lines.line_colored(p2, p3, duration, color);
    lines.line_colored(p3, p4, duration, color);
    lines.line_colored(p4, p1, duration, color);
}

fn add_circle(
    lines: &mut DebugLines,
    origin: Vec3,
    rot: Quat,
    radius: f32,
    duration: f32,
    color: Color,
) {
    let x_rotate = Quat::from_rotation_x(PI);
    add_semicircle(lines, origin, rot, radius, duration, color);
    add_semicircle(lines, origin, rot * x_rotate, radius, duration, color);
}

fn add_semicircle(
    lines: &mut DebugLines,
    origin: Vec3,
    rot: Quat,
    radius: f32,
    duration: f32,
    color: Color,
) {
    let x_rotate = Quat::from_rotation_y(PI);
    add_quartercircle(lines, origin, rot, radius, duration, color);
    add_quartercircle(lines, origin, rot * x_rotate, radius, duration, color);
}

fn add_quartercircle(
    lines: &mut DebugLines,
    origin: Vec3,
    rot: Quat,
    radius: f32,
    duration: f32,
    color: Color,
) {
    let quarter_circle_segments = 4;
    let angle = FRAC_PI_2 / quarter_circle_segments as f32;
    let mut current_point = rot.mul_vec3(Vec3::X * radius);
    let direction = Quat::from_axis_angle(rot.mul_vec3(Vec3::Y), angle);
    for _ in 0..quarter_circle_segments {
        let next_point = direction.mul_vec3(current_point);
        lines.line_colored(origin + current_point, origin + next_point, duration, color);
        current_point = next_point;
    }
}

use bevy::{prelude::*, render::camera::ScalingMode, window::WindowResolution};
use bevy_mod_picking::*;
mod pieces;
use pieces::*;
mod board;
use board::*;
mod ui;
use ui::*;

fn main() {
    App::new()
        // Default aliasing use 4 samples
        .insert_resource(Msaa::Sample4)
        // Set WindowDescriptor Resource to change title and size
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                resolution: WindowResolution::new(1920., 1080.).into(),
                title: "Aryeh's chess".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugins(DefaultPickingPlugins)
        .add_plugin(BoardPlugin)
        .add_plugin(PiecesPlugin)
        .add_plugin(UIPlugin)
        .add_startup_system(setup.in_base_set(StartupSet::PostStartup))
        .run();
}

fn setup(mut commands: Commands) {
    // Camera
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_matrix(Mat4::from_rotation_translation(
                Quat::from_xyzw(-0.3, -0.5, -0.3, 0.5).normalize(),
                Vec3::new(-7.0, 20.0, 4.0),
            )),
            projection: OrthographicProjection {
                scale: 3.0,
                scaling_mode: ScalingMode::FixedVertical(5.0),
                ..Default::default()
            }
            .into(),
            ..Default::default()
        })
        .insert(PickingCameraBundle::default());

    // Light
    commands.spawn(PointLightBundle {
        transform: Transform::from_translation(Vec3::new(4.0, 8.0, 4.0)),
        ..Default::default()
    });
}

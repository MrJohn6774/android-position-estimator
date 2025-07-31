#![allow(clippy::type_complexity)]

#[cfg(target_os = "android")]
mod ffi;
mod plugins;

#[cfg(target_os = "android")]
use bevy::render::settings::Backends;
use bevy::{
    prelude::*,
    render::{RenderPlugin, settings::WgpuSettings},
    window::WindowMode,
    winit::WinitSettings,
};
use bevy_debug_text_overlay::OverlayPlugin;
use bevy_infinite_grid::{InfiniteGridBundle, InfiniteGridPlugin};
use bevy_screen_diagnostics::{ScreenDiagnosticsPlugin, ScreenFrameDiagnosticsPlugin};
#[cfg(target_os = "android")]
use plugins::sensor::SensorPlugin;
use plugins::{camera::AppCameraPlugin, state::StatePlugin};

#[bevy_main]
pub fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(RenderPlugin {
                render_creation: WgpuSettings {
                    #[cfg(target_os = "android")]
                    backends: Some(Backends::VULKAN),
                    ..default()
                }
                .into(),
                synchronous_pipeline_compilation: false,
                ..default()
            })
            .set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: true,
                    mode: WindowMode::Windowed,
                    ..default()
                }),
                ..default()
            }),
    )
    // .add_plugins(InfiniteGridPlugin)
    .add_plugins(ScreenDiagnosticsPlugin::default())
    .add_plugins(ScreenFrameDiagnosticsPlugin)
    // .add_plugins(OverlayPlugin::default())
    .add_plugins((AppCameraPlugin, StatePlugin))
    .add_systems(Startup, (setup_scene));

    #[cfg(target_os = "android")]
    {
        app.add_plugins(SensorPlugin);
        app.insert_resource(WinitSettings::mobile());
    }

    app.run();
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // grid
    // commands.spawn(InfiniteGridBundle::default());
    // plane
    commands.spawn((
        Mesh3d(meshes.add(Plane3d::default().mesh().size(5.0, 5.0))),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.2, 0.1))),
    ));
    // cube
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(Color::srgb(0.5, 0.4, 0.3))),
        Transform::from_xyz(0.0, 0.5, 0.0),
    ));
    // sphere
    commands.spawn((
        Mesh3d(meshes.add(Sphere::new(0.5).mesh().ico(4).unwrap())),
        MeshMaterial3d(materials.add(Color::srgb(0.1, 0.4, 0.8))),
        Transform::from_xyz(1.5, 1.5, 1.5),
    ));
    // light
    commands.spawn((
        PointLight {
            intensity: 1_000_000.0,
            // Shadows makes some Android devices segfault, this is under investigation
            // https://github.com/bevyengine/bevy/issues/8214
            #[cfg(not(target_os = "android"))]
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));
}

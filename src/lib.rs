#![allow(clippy::type_complexity)]

#[cfg(target_os = "android")]
mod ffi;
mod plugins;

#[cfg(target_os = "android")]
use bevy::render::settings::Backends;
use bevy::{
    prelude::*,
    render::{settings::WgpuSettings, RenderPlugin},
    window::WindowMode,
};
use plugins::camera::AppCameraPlugin;
#[cfg(target_os = "android")]
use plugins::sensor::SensorPlugin;

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
    .add_plugins(AppCameraPlugin)
    .add_systems(Startup, (setup_scene))
    .add_systems(Update, (button_handler));

    #[cfg(target_os = "android")]
    app.add_plugins(SensorPlugin);

    // MSAA makes some Android devices panic, this is under investigation
    // https://github.com/bevyengine/bevy/issues/8229
    #[cfg(target_os = "android")]
    app.insert_resource(Msaa::Off);

    app.run();
}

/// set up a simple 3D scene
fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // plane
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(5.0).into()),
        material: materials.add(Color::rgb(0.1, 0.2, 0.1).into()),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
        material: materials.add(Color::rgb(0.5, 0.4, 0.3).into()),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                subdivisions: 4,
                radius: 0.5,
            })
            .unwrap(),
        ),
        material: materials.add(Color::rgb(0.1, 0.4, 0.8).into()),
        transform: Transform::from_xyz(1.5, 1.5, 1.5),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        point_light: PointLight {
            intensity: 5000.0,
            // Shadows makes some Android devices segfault, this is under investigation
            // https://github.com/bevyengine/bevy/issues/8214
            #[cfg(not(target_os = "android"))]
            shadows_enabled: true,
            ..default()
        },
        ..default()
    });

    // Test ui
    commands
        .spawn(ButtonBundle {
            style: Style {
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position_type: PositionType::Absolute,
                left: Val::Px(50.0),
                right: Val::Px(50.0),
                bottom: Val::Px(50.0),
                ..default()
            },
            ..default()
        })
        .with_children(|b| {
            b.spawn(
                TextBundle::from_section(
                    "Test Button",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::BLACK,
                        ..default()
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

fn button_handler(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = Color::BLUE.into();
            }
            Interaction::Hovered => {
                *color = Color::GRAY.into();
            }
            Interaction::None => {
                *color = Color::WHITE.into();
            }
        }
    }
}

// Pause audio when app goes into background and resume when it returns.
// This is handled by the OS on iOS, but not on Android.
// fn handle_lifetime(
//     mut lifetime_events: EventReader<ApplicationLifetime>,
//     music_controller: Query<&AudioSink>,
// ) {
//     for event in lifetime_events.read() {
//         match event {
//             ApplicationLifetime::Suspended => music_controller.single().pause(),
//             ApplicationLifetime::Resumed => music_controller.single().play(),
//             ApplicationLifetime::Started => (),
//         }
//     }
// }

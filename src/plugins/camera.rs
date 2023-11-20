#![allow(clippy::type_complexity)]

use bevy::{input::touch, prelude::*, time::Time};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct AppCameraPlugin;

impl Plugin for AppCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
            .add_systems(PostStartup, setup)
            .add_systems(Update, touch_control)
            .insert_resource(TouchTracker::default())
            .insert_resource(TouchConfig::default());
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default(),
    ));
}

#[derive(PartialEq, Default)]
enum GestureType {
    #[default]
    None,
    Pan,
    Pinch,
    PinchCancelled,
}

/// Contains the configuration parameters for the plugin.
/// A copy of this will be attached as a `Resource` to the `App`.
#[derive(Resource, Clone)]
pub struct TouchConfig {
    /// How far the camera will move relative to the touch drag distance. Higher is faster
    pub drag_sensitivity: f32,
    /// How much the camera will zoom relative to the pinch distance using two fingers. Higher means faster.
    /// At the moment the default is very low at 0.005 but this might change in the future
    pub zoom_sensitivity: f32,
    /// Minimum time before starting to pan in seconds. Useful to avoid panning on short taps
    pub touch_time_min: f32,
    /// Tolerance for pinch fingers moving in opposite directions (-1. .. 1.).
    /// Higher values make it more tolerant.
    /// Very low values not recommended as it would be overly sensitive
    pub opposites_tolerance: f32,
}

impl Default for TouchConfig {
    fn default() -> Self {
        Self {
            drag_sensitivity: 0.005,
            zoom_sensitivity: 0.05,
            touch_time_min: 0.01,
            opposites_tolerance: 0.,
        }
    }
}

#[derive(Resource, Default)]
struct TouchTracker {
    pub time_start_touch: f32,
    pub gesture_type: GestureType,

    // Keeps track of position on last frame.
    // This is different from Touch.last_position as that only updates when there has been a movement
    pub last_touch_a: Option<Vec2>,
    pub last_touch_b: Option<Vec2>,
}

fn touch_control(
    touches_res: Res<Touches>,
    mut orbit_camera: Query<&mut PanOrbitCamera>,
    mut touch_tracker: ResMut<TouchTracker>,
    config: Res<TouchConfig>,
    time: Res<Time>,
) {
    let Ok(mut pan_orbit) = orbit_camera.get_single_mut() else {
        return;
    };

    let touches: Vec<&touch::Touch> = touches_res.iter().collect();

    if touches.is_empty() {
        touch_tracker.gesture_type = GestureType::None;
        touch_tracker.last_touch_a = None;
        touch_tracker.last_touch_b = None;
        return;
    }

    if touches_res.any_just_released() {
        touch_tracker.gesture_type = GestureType::PinchCancelled;
        touch_tracker.last_touch_a = None;
        touch_tracker.last_touch_b = None;
    }

    if touches.len() == 2 {
        touch_tracker.gesture_type = GestureType::Pinch;
        // complicated way to reset previous position to prevent some bugs. Should simplify
        let last_a = if touch_tracker.last_touch_b.is_none() {
            touches[0].position()
        } else {
            touch_tracker.last_touch_a.unwrap_or(touches[0].position())
        };
        let last_b = if touch_tracker.last_touch_b.is_none() {
            touches[1].position()
        } else {
            touch_tracker.last_touch_b.unwrap_or(touches[1].position())
        };

        let delta_a = touches[0].position() - last_a;
        let delta_b = touches[1].position() - last_b;
        let delta_total = (delta_a + delta_b).length();
        let dot_delta = delta_a.dot(delta_b);
        if dot_delta > config.opposites_tolerance {
            return;
        }

        let distance_current = touches[0].position() - touches[1].position();
        let distance_prev = touches[0].previous_position() - touches[1].previous_position();
        let pinch_direction = distance_prev.length() - distance_current.length();

        pan_orbit.target_radius += pinch_direction.signum() * delta_total * config.zoom_sensitivity;

        touch_tracker.last_touch_a = Some(touches[0].position());
        touch_tracker.last_touch_b = Some(touches[1].position());
    } else if touches.len() == 1
        && matches!(
            touch_tracker.gesture_type,
            GestureType::None | GestureType::Pan
        )
    {
        if touch_tracker.gesture_type == GestureType::None {
            touch_tracker.time_start_touch = time.elapsed_seconds();
        }
        touch_tracker.gesture_type = GestureType::Pan;
        let time_since_start = time.elapsed_seconds() - touch_tracker.time_start_touch;
        if time_since_start < config.touch_time_min {
            return;
        }

        pan_orbit.target_alpha -= touches[0].delta().x * config.drag_sensitivity;
        pan_orbit.target_beta += touches[0].delta().y * config.drag_sensitivity;

        touch_tracker.last_touch_a = Some(touches[0].position());
        touch_tracker.last_touch_b = None;
    }
}

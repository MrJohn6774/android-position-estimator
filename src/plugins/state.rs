use bevy::prelude::*;
use bevy_debug_text_overlay::screen_print;

#[cfg(target_os = "android")]
use super::sensor::SensorData;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(StateVector::default());

        #[cfg(target_os = "android")]
        app.add_systems(Update, update_state_vector);
        app.add_systems(PostUpdate, print_state);
    }
}

#[derive(Debug, Default, Resource)]
pub struct StateVector {
    position: Vec3,
    velocity: Vec3,
    orientation: Quat,
    rotation: Quat,
}

#[cfg(target_os = "android")]
fn update_state_vector(sensor_data: Res<SensorData>, mut states: ResMut<StateVector>) {
    // Integrate (trapezoidal rule, backwards in time) acceleration -> velocity -> position
    let accel_t = sensor_data.accelerometer.latest().unwrap();

    let accel_t_minus_1_event = sensor_data.accelerometer.t_minus(1);

    let accel_t_minus_2_event = sensor_data.accelerometer.t_minus(2);

    let vel_t_zero = states.velocity.clone();

    if let Some(accel_t_minus_1) = accel_t_minus_1_event {
        states.velocity += (*accel_t.values.vec3().unwrap()
            + *accel_t_minus_1.values.vec3().unwrap())
            * ((accel_t.timestamp - accel_t_minus_1.timestamp) as f32)
            * 1e-9
            * 0.5;
    }

    if let Some(accel_t_minus_2) = accel_t_minus_2_event {
        let vel_t_plus_1 = states.velocity.clone();
        states.position += (vel_t_zero + vel_t_plus_1)
            * ((accel_t.timestamp - accel_t_minus_2.timestamp) as f32)
            * 1e-9
            * 0.25;
    }

    // Complementary filter: rot vec + mag vec
}

fn print_state(states: Res<StateVector>) {
    screen_print!("Velocity: {:?}", states.velocity);
    screen_print!("Position: {:?}", states.position);
}

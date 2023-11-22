use bevy::prelude::*;
use bevy::window::ApplicationLifetime;
use bevy_debug_text_overlay::screen_print;

use crate::ffi::sensor::{Sensor, SensorEvent, SensorEventQueue, SensorManager, SensorType};

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SensorData::default())
            .insert_non_send_resource(Sensors::default())
            .add_systems(PostStartup, setup_sensors)
            .add_systems(
                Update,
                (
                    handle_lifetime,
                    (update_sensor_data, print_sensor_data).chain(),
                ),
            );
    }
}

struct Sensors {
    // manager: Option<SensorManager>,
    queue: Option<SensorEventQueue>,
    sensors: Vec<Sensor>,
}

#[derive(Debug, Default, Resource)]
struct SensorData {
    accelerometer: SensorEvent,
    gyroscope: SensorEvent,
    rotation: SensorEvent,
    compass: SensorEvent,
    gravity: SensorEvent,
}

impl Default for Sensors {
    fn default() -> Self {
        Self {
            // manager: None,
            queue: None,
            sensors: Vec::new(),
        }
    }
}

impl Sensors {
    const SAMPLING_PERIOD: i32 = 1_000_000 / 10; // microseconds (10 Hz)

    fn enable(&self) {
        dbg!("Enabling sensors...");
        self.sensors.iter().for_each(|sensor| {
            self.queue
                .as_ref()
                .unwrap()
                .enable_sensor(&sensor, Self::SAMPLING_PERIOD);
        })
    }

    fn get_events(&self) -> Vec<SensorEvent> {
        if let Some(queue) = &self.queue {
            queue.get_events()
        } else {
            warn!("Sensor event queue not initialized!");
            Vec::new()
        }
    }

    fn disable(&self) {
        dbg!("Disabling sensors...");
        self.sensors.iter().for_each(|sensor| {
            self.queue.as_ref().unwrap().disable_sensor(&sensor);
        })
    }
}

fn setup_sensors(mut sensors: NonSendMut<Sensors>) {
    let manager = SensorManager::new();
    let queue = manager.create_event_queue();

    [
        SensorType::Accelerometer,
        SensorType::Gyroscope,
        SensorType::Rotation,
        SensorType::Compass,
        SensorType::Gravity,
    ]
    .iter()
    .for_each(|&sensor_type| {
        sensors
            .sensors
            .push(manager.get_default_sensor(sensor_type));
    });

    // sensors.manager = Some(manager);
    sensors.queue = Some(queue);
}

fn handle_lifetime(
    mut lifetime_events: EventReader<ApplicationLifetime>,
    sensors: NonSend<Sensors>,
) {
    for event in lifetime_events.read() {
        match event {
            ApplicationLifetime::Resumed => sensors.enable(),
            ApplicationLifetime::Suspended => sensors.disable(),
            ApplicationLifetime::Started => sensors.enable(),
        }
    }
}

fn update_sensor_data(sensors: NonSend<Sensors>, mut sensor_data: ResMut<SensorData>) {
    let events = sensors.get_events();
    screen_print!("Sensor queue length: {}", &events.len());
    events.iter().for_each(|event| {
        match event.sensor_type {
            SensorType::Accelerometer => sensor_data.accelerometer = event.clone(),
            SensorType::Gyroscope => sensor_data.gyroscope = event.clone(),
            SensorType::Rotation => sensor_data.rotation = event.clone(),
            SensorType::Compass => sensor_data.compass = event.clone(),
            SensorType::Gravity => sensor_data.gravity = event.clone(),
            _ => (),
        };
    });
}

fn print_sensor_data(sensor_data: Res<SensorData>) {
    screen_print!("Accel: {:?}", sensor_data.accelerometer.values);
    screen_print!("Gyro: {:?}", sensor_data.gyroscope.values);
    screen_print!("Rotation: {:?}", sensor_data.rotation.values);
    screen_print!("Compass: {:?}", sensor_data.compass.values);
    screen_print!("Gravity: {:?}", sensor_data.gravity.values);
}

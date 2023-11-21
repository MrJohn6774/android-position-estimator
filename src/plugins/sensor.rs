use bevy::prelude::*;
use bevy::window::ApplicationLifetime;

use crate::ffi::sensor::{Sensor, SensorEventQueue, SensorManager, SensorType};

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SensorData::default())
            .insert_non_send_resource(Sensors::default())
            .add_systems(PostStartup, setup_sensors)
            .add_systems(Update, (handle_lifetime, update_sensor_data));
    }
}

struct Sensors {
    // manager: Option<SensorManager>,
    queue: Option<SensorEventQueue>,
    sensors: Vec<Sensor>,
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
    const SAMPLING_PERIOD: i32 = 1_000_000 / 10; // microseconds (50Hz)

    fn enable(&self) {
        dbg!("Enabling sensors...");
        self.sensors.iter().for_each(|sensor| {
            self.queue
                .as_ref()
                .unwrap()
                .enable_sensor(&sensor, Self::SAMPLING_PERIOD);
        })
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

    [SensorType::Accelerometer].iter().for_each(|&sensor_type| {
        sensors
            .sensors
            .push(manager.get_default_sensor(sensor_type));
    });

    // sensors.manager = Arc::new(Some(manager));
    sensors.queue = Some(queue);
    sensors.enable();
}

fn handle_lifetime(
    mut lifetime_events: EventReader<ApplicationLifetime>,
    sensors: NonSend<Sensors>,
) {
    for event in lifetime_events.read() {
        match event {
            ApplicationLifetime::Resumed => sensors.enable(),
            ApplicationLifetime::Suspended => sensors.disable(),
            ApplicationLifetime::Started => (),
        }
    }
}

#[derive(Resource, Debug)]
pub struct SensorData {
    pub acceleration: Vec3,
    pub quaternion: Quat,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            acceleration: Vec3::ZERO,
            quaternion: Quat::default(),
        }
    }
}

fn update_sensor_data(sensors: NonSend<Sensors>, mut sensor_data: ResMut<SensorData>) {
    let events = sensors.queue.as_ref().unwrap().get_events();

    dbg!(events).iter().for_each(|event| {
        match event.sensor_type {
            SensorType::Accelerometer => sensor_data.acceleration = Vec3::from_slice(&event.values),
            SensorType::Gyroscope => todo!(),
            SensorType::Compass => todo!(),
        }
    });
}

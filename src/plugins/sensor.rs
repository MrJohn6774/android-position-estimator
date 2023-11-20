use std::sync::Arc;

use bevy::prelude::*;
use bevy::window::ApplicationLifetime;

use crate::ffi::sensor::{Sensor, SensorEventQueue, SensorManager, SensorType};

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SensorData::default())
            .insert_resource(Sensors::default())
            .add_systems(PostStartup, setup_sensors)
            .add_systems(Update, (handle_lifetime, update_sensor_data));
    }
}

#[derive(Resource)]
struct Sensors {
    // manager: Arc<Option<SensorManager>>,
    queue: Arc<Option<SensorEventQueue>>,
    sensors: Vec<Arc<Sensor>>,
}

impl Default for Sensors {
    fn default() -> Self {
        Self {
            // manager: Arc::new(None),
            queue: Arc::new(None),
            sensors: Vec::new(),
        }
    }
}

impl Sensors {
    const SAMPLING_PERIOD: i32 = 1_000_000 / 50; // microseconds (50Hz)

    fn enable(&self) {
        self.sensors.iter().for_each(|sensor| {
            self.queue
                .as_ref()
                .as_ref()
                .unwrap()
                .enable_sensor(&sensor, Self::SAMPLING_PERIOD);
        })
    }

    fn disable(&self) {
        self.sensors.iter().for_each(|sensor| {
            self.queue
                .as_ref()
                .as_ref()
                .unwrap()
                .disable_sensor(&sensor);
        })
    }
}

fn setup_sensors(mut sensors: ResMut<Sensors>) {
    let manager = SensorManager::new();
    let queue = manager.create_event_queue();

    [SensorType::Accelerometer].iter().for_each(|&sensor_type| {
        sensors
            .sensors
            .push(Arc::new(manager.get_default_sensor(sensor_type)));
    });

    // sensors.manager = Arc::new(Some(manager));
    sensors.queue = Arc::new(Some(queue));
}

fn handle_lifetime(mut lifetime_events: EventReader<ApplicationLifetime>, sensors: Res<Sensors>) {
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

fn update_sensor_data(sensors: Res<Sensors>, sensor_data: ResMut<SensorData>) {
    let events = sensors.queue.as_ref().as_ref().unwrap().get_events();
    dbg!(events);
}

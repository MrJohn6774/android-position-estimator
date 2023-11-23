use bevy::prelude::*;
use bevy::window::ApplicationLifetime;
use bevy_debug_text_overlay::screen_print;
use std::collections::VecDeque;

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
    const SAMPLING_PERIOD: i32 = 1_000_000 / 50; // microseconds (50 Hz)

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

#[derive(Debug)]
struct SensorDataSeries {
    series: VecDeque<SensorEvent>,
    size: usize,
}

impl SensorDataSeries {
    const MIN_DELTA_TIME: i64 = Sensors::SAMPLING_PERIOD as i64 * 1_000; // nanoseconds

    pub fn new(size: usize) -> Self {
        let mut series = VecDeque::with_capacity(size);
        series.push_back(SensorEvent::default());

        Self { series, size }
    }

    pub fn add(&mut self, sensor_event: SensorEvent) -> Option<SensorEvent> {
        // newest data lives at the back, oldest at the front
        let mut expired_data = None;

        if self.series.len() >= self.size {
            expired_data = self.series.pop_front();
        }

        if sensor_event.timestamp - self.latest().unwrap().timestamp >= Self::MIN_DELTA_TIME {
            self.series.push_back(sensor_event);
        }

        expired_data
    }

    pub fn t_minus(&self, index: usize) -> Option<&SensorEvent> {
        if index >= self.size {
            warn!("Invalid to access to SensorDataSeries with index {}", index);
            return None;
        }

        self.series.get(self.series.len() - index - 1)
    }

    pub fn latest(&self) -> Option<&SensorEvent> {
        self.series.back()
    }

    pub fn oldest(&self) -> Option<&SensorEvent> {
        self.series.front()
    }
}

#[derive(Debug, Resource)]
struct SensorData {
    accelerometer: SensorDataSeries,
    gyroscope: SensorDataSeries,
    rotation: SensorDataSeries,
    compass: SensorDataSeries,
    gravity: SensorDataSeries,
}

impl Default for SensorData {
    fn default() -> Self {
        Self {
            accelerometer: SensorDataSeries::new(5),
            gyroscope: SensorDataSeries::new(5),
            rotation: SensorDataSeries::new(5),
            compass: SensorDataSeries::new(5),
            gravity: SensorDataSeries::new(5),
        }
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
            SensorType::Accelerometer => sensor_data.accelerometer.add(event.clone()),
            SensorType::Gyroscope => sensor_data.gyroscope.add(event.clone()),
            SensorType::Rotation => sensor_data.rotation.add(event.clone()),
            SensorType::Compass => sensor_data.compass.add(event.clone()),
            SensorType::Gravity => sensor_data.gravity.add(event.clone()),
            _ => None,
        };
    });
}

fn print_sensor_data(sensor_data: Res<SensorData>) {
    screen_print!(
        "Accel: {:?}",
        sensor_data.accelerometer.latest().unwrap().values
    );
    screen_print!("Gyro: {:?}", sensor_data.gyroscope.latest().unwrap().values);
    screen_print!(
        "Rotation: {:?}",
        sensor_data.rotation.latest().unwrap().values
    );
    screen_print!(
        "Compass: {:?}",
        sensor_data.compass.latest().unwrap().values
    );
    screen_print!(
        "Gravity: {:?}",
        sensor_data.gravity.latest().unwrap().values
    );
}

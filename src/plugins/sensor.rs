use bevy::prelude::*;
use bevy::window::AppLifecycle;
use bevy_debug_text_overlay::screen_print;
use std::collections::VecDeque;

use crate::ffi::sensor::{
    Sensor, SensorEvent, SensorEventQueue, SensorManager, SensorType, SensorValues,
};

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
pub struct SensorDataSeries {
    series: VecDeque<SensorEvent>,
    size: usize,
    lp_alpha: f32,
}

impl SensorDataSeries {
    const MIN_DELTA_TIME: i64 = Sensors::SAMPLING_PERIOD as i64 * 1_000; // nanoseconds

    pub fn new(size: usize) -> Self {
        let mut series = VecDeque::with_capacity(size);
        series.push_back(SensorEvent::default());

        Self {
            series,
            size,
            lp_alpha: 0.2738, // 3Hz filter
        }
    }

    pub fn add(&mut self, mut sensor_event: SensorEvent) -> Option<SensorEvent> {
        // newest data lives at the back, oldest at the front
        let mut expired_data = None;

        if self.series.len() >= self.size {
            expired_data = self.series.pop_front();
        }

        if sensor_event.timestamp - self.latest().unwrap().timestamp >= Self::MIN_DELTA_TIME {
            // low-pass filter
            sensor_event.values = match (self.latest().unwrap().values, sensor_event.values) {
                (SensorValues::Vec3(vector_latest), SensorValues::Vec3(vector_new)) => {
                    SensorValues::Vec3(
                        vector_latest * (1. - self.lp_alpha) + vector_new * self.lp_alpha,
                    )
                }
                (SensorValues::Quat(quat_latest), SensorValues::Quat(quat_new)) => {
                    SensorValues::Quat(
                        quat_latest * (1. - self.lp_alpha) + quat_new * self.lp_alpha,
                    )
                }
                _ => sensor_event.values,
            };

            self.series.push_back(sensor_event);
        }

        expired_data
    }

    pub fn t_minus(&self, index: usize) -> Option<&SensorEvent> {
        if index >= self.size {
            warn!("Invalid to access to SensorDataSeries with index {}", index);
            return None;
        } else if index >= self.series.len() {
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
pub struct SensorData {
    pub accelerometer: SensorDataSeries,
    pub gyroscope: SensorDataSeries,
    pub rotation: SensorDataSeries,
    pub compass: SensorDataSeries,
    pub gravity: SensorDataSeries,
}

impl SensorData {
    fn add_event(&mut self, event: SensorEvent) {
        match event.sensor_type {
            SensorType::Accelerometer => self.accelerometer.add(event),
            SensorType::Gyroscope => self.gyroscope.add(event),
            SensorType::Rotation => self.rotation.add(event),
            SensorType::Compass => self.compass.add(event),
            SensorType::Gravity => self.gravity.add(event),
            _ => None,
        };
    }
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

fn handle_lifetime(mut lifetime_events: EventReader<AppLifecycle>, sensors: NonSend<Sensors>) {
    for event in lifetime_events.read() {
        match event {
            AppLifecycle::Idle => sensors.disable(),
            AppLifecycle::Running => sensors.enable(),
            AppLifecycle::WillSuspend => sensors.disable(),
            AppLifecycle::Suspended => sensors.disable(),
            AppLifecycle::WillResume => sensors.enable(),
        }
    }
}

fn update_sensor_data(sensors: NonSend<Sensors>, mut sensor_data: ResMut<SensorData>) {
    let events = sensors.get_events();
    // screen_print!("Sensor queue length: {}", &events.len());
    events.iter().for_each(|event| {
        sensor_data.add_event(event.clone());
    });
}

fn print_sensor_data(sensor_data: Res<SensorData>) {
    // screen_print!(
    //     "Accel: {:?}",
    //     sensor_data.accelerometer.latest().unwrap().values
    // );
    // screen_print!("Gyro: {:?}", sensor_data.gyroscope.latest().unwrap().values);
    // screen_print!(
    //     "Rotation: {:?}",
    //     sensor_data.rotation.latest().unwrap().values
    // );
    // screen_print!(
    //     "Compass: {:?}",
    //     sensor_data.compass.latest().unwrap().values
    // );
    // screen_print!(
    //     "Gravity: {:?}",
    //     sensor_data.gravity.latest().unwrap().values
    // );
}

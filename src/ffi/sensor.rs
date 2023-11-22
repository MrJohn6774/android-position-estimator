#![allow(dead_code)]

use bevy::log::warn;
use ndk_sys::{
    ALooper_pollAll, ALooper_prepare, ASensor, ASensorEvent, ASensorEventQueue,
    ASensorEventQueue_disableSensor, ASensorEventQueue_enableSensor, ASensorEventQueue_getEvents,
    ASensorEventQueue_setEventRate, ASensorManager, ASensorManager_createEventQueue,
    ASensorManager_destroyEventQueue, ASensorManager_getDefaultSensor, ASensorManager_getInstance,
    ALOOPER_PREPARE_ALLOW_NON_CALLBACKS, ASENSOR_STATUS_ACCURACY_HIGH, ASENSOR_STATUS_ACCURACY_LOW,
    ASENSOR_STATUS_ACCURACY_MEDIUM, ASENSOR_STATUS_NO_CONTACT, ASENSOR_STATUS_UNRELIABLE,
    ASENSOR_TYPE_ACCELEROMETER, ASENSOR_TYPE_ADDITIONAL_INFO,
    ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR, ASENSOR_TYPE_GRAVITY, ASENSOR_TYPE_GYROSCOPE,
    ASENSOR_TYPE_ROTATION_VECTOR,
};
use num_derive::FromPrimitive;

#[derive(Clone, Debug, FromPrimitive)]
pub enum SensorAccuracy {
    High = ASENSOR_STATUS_ACCURACY_HIGH as isize,
    Low = ASENSOR_STATUS_ACCURACY_LOW as isize,
    Medium = ASENSOR_STATUS_ACCURACY_MEDIUM as isize,
    NoContact = ASENSOR_STATUS_NO_CONTACT as isize,
    Unreliable = ASENSOR_STATUS_UNRELIABLE as isize,
}

#[derive(Clone, Copy, Debug, FromPrimitive)]
pub enum SensorType {
    Accelerometer = ASENSOR_TYPE_ACCELEROMETER as isize,
    Gyroscope = ASENSOR_TYPE_GYROSCOPE as isize,
    Rotation = ASENSOR_TYPE_ROTATION_VECTOR as isize,
    Compass = ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR as isize,
    Gravity = ASENSOR_TYPE_GRAVITY as isize,
    AdditionalInfo = ASENSOR_TYPE_ADDITIONAL_INFO as isize,
    Unavailable = 0,
}

pub struct Sensor {
    sensor: *const ASensor,
}

pub struct SensorManager {
    manager: *mut ASensorManager,
}

#[derive(Clone, Debug)]
pub struct SensorEvent {
    pub accuracy: SensorAccuracy,
    pub sensor_type: SensorType,
    pub timestamp: i64,
    pub values: Vec<f32>,
}

#[derive(Debug)]
pub struct SensorEventQueue {
    queue: *mut ASensorEventQueue,
}

impl SensorManager {
    pub fn new() -> Self {
        let manager = unsafe { ASensorManager_getInstance() };
        assert!(!manager.is_null(), "*mut ASensorManger is null");
        Self { manager }
    }

    pub fn get_default_sensor(&self, sensor_type: SensorType) -> Sensor {
        let sensor = unsafe { ASensorManager_getDefaultSensor(self.manager, sensor_type as i32) };
        assert!(!sensor.is_null(), "*const ASensor is null");
        Sensor { sensor }
    }

    pub fn create_event_queue(&self) -> SensorEventQueue {
        let looper_ptr = unsafe { ALooper_prepare(ALOOPER_PREPARE_ALLOW_NON_CALLBACKS as _) };
        assert!(!looper_ptr.is_null(), "*mut ALooper is null");
        let queue = unsafe {
            // ident field has to be 2
            // (https://github.com/rust-mobile/android-activity/blob/9fce89021959a6f6ea8853221367bfa305803369/android-activity/src/native_activity/mod.rs#L290)
            ASensorManager_createEventQueue(self.manager, looper_ptr, 2, None, std::ptr::null_mut())
        };
        assert!(!queue.is_null(), "*mut ASensorEventQueue is null");
        SensorEventQueue { queue }
    }

    pub fn destroy_event_queue(&self, queue: SensorEventQueue) {
        let status = unsafe { ASensorManager_destroyEventQueue(self.manager, queue.queue) };
        assert!(status >= 0);
    }
}

impl Default for SensorEvent {
    fn default() -> Self {
        Self {
            accuracy: SensorAccuracy::NoContact,
            sensor_type: SensorType::Accelerometer,
            timestamp: 0,
            values: vec![0., 0., 0.],
        }
    }
}

impl SensorEventQueue {
    pub fn enable_sensor(&self, sensor: &Sensor, sampling_period_us: i32) {
        let status = unsafe { ASensorEventQueue_enableSensor(self.queue, sensor.sensor) };
        assert!(status >= 0);
        let status = unsafe {
            ASensorEventQueue_setEventRate(self.queue, sensor.sensor, sampling_period_us)
        };
        assert!(status >= 0);
    }

    pub fn get_events(&self) -> Vec<SensorEvent> {
        let mut fd = -1;
        let mut events = -1;
        let mut data = std::ptr::null_mut();
        let status = unsafe {
            // non-blocking
            ALooper_pollAll(0, &mut fd, &mut events, &mut data)
        };
        assert_ne!(status, 0);

        let mut event: ASensorEvent = unsafe { std::mem::zeroed() };
        let mut event_count =
            unsafe { ASensorEventQueue_getEvents(self.queue, &mut event as *mut _, 1) };
        assert!(event_count >= 0 && event_count <= 1);

        let mut events: Vec<SensorEvent> = Vec::new();
        loop {
            if let Some(sensor_type) = num::FromPrimitive::from_i32(event.type_) {
                match sensor_type {
                    SensorType::Accelerometer => {
                        events.push(SensorEvent {
                            accuracy: num::FromPrimitive::from_i8(unsafe {
                                event.__bindgen_anon_1.__bindgen_anon_1.acceleration.status
                            })
                            .unwrap_or(SensorAccuracy::Unreliable),
                            sensor_type: SensorType::Accelerometer,
                            timestamp: event.timestamp,
                            values: unsafe {
                                vec![
                                    event
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .acceleration
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .x,
                                    event
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .acceleration
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .y,
                                    event
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .acceleration
                                        .__bindgen_anon_1
                                        .__bindgen_anon_1
                                        .z,
                                ]
                            },
                        });
                    }
                    SensorType::Gyroscope => events.push(SensorEvent {
                        accuracy: num::FromPrimitive::from_i8(unsafe {
                            event.__bindgen_anon_1.__bindgen_anon_1.gyro.status
                        })
                        .unwrap_or(SensorAccuracy::Unreliable),
                        sensor_type: SensorType::Gyroscope,
                        timestamp: event.timestamp,
                        values: unsafe {
                            vec![
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .gyro
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .x,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .gyro
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .y,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .gyro
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .z,
                            ]
                        },
                    }),
                    SensorType::Rotation => events.push(SensorEvent {
                        accuracy: num::FromPrimitive::from_i8(unsafe {
                            event.__bindgen_anon_1.__bindgen_anon_1.vector.status
                        })
                        .unwrap_or(SensorAccuracy::Unreliable),
                        sensor_type: SensorType::Rotation,
                        timestamp: event.timestamp,
                        values: unsafe {
                            vec![
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .x,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .y,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .z,
                            ]
                        },
                    }),
                    SensorType::Compass => events.push(SensorEvent {
                        accuracy: num::FromPrimitive::from_i8(unsafe {
                            event.__bindgen_anon_1.__bindgen_anon_1.vector.status
                        })
                        .unwrap_or(SensorAccuracy::Unreliable),
                        sensor_type: SensorType::Compass,
                        timestamp: event.timestamp,
                        values: unsafe {
                            vec![
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .x,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .y,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .z,
                            ]
                        },
                    }),
                    SensorType::Gravity => events.push(SensorEvent {
                        accuracy: num::FromPrimitive::from_i8(unsafe {
                            event.__bindgen_anon_1.__bindgen_anon_1.vector.status
                        })
                        .unwrap_or(SensorAccuracy::Unreliable),
                        sensor_type: SensorType::Gravity,
                        timestamp: event.timestamp,
                        values: unsafe {
                            vec![
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .x,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .y,
                                event
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .vector
                                    .__bindgen_anon_1
                                    .__bindgen_anon_1
                                    .z,
                            ]
                        },
                    }),
                    _ => (),
                }
            } else {
                warn!("Sensor (type: {}) not recognized!", event.type_);
            }

            event_count =
                unsafe { ASensorEventQueue_getEvents(self.queue, &mut event as *mut _, 1) };

            if event_count < 1 {
                break;
            }
        }
        events
    }

    pub fn disable_sensor(&self, sensor: &Sensor) {
        unsafe { ASensorEventQueue_disableSensor(self.queue, sensor.sensor) };
    }
}

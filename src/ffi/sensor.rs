#![allow(dead_code)]

use ndk_sys::{
    ALooper_pollAll, ALooper_prepare, ASensor, ASensorEvent, ASensorEventQueue,
    ASensorEventQueue_disableSensor, ASensorEventQueue_enableSensor, ASensorEventQueue_getEvents,
    ASensorEventQueue_setEventRate, ASensorManager, ASensorManager_createEventQueue,
    ASensorManager_destroyEventQueue, ASensorManager_getDefaultSensor, ASensorManager_getInstance,
    ALOOPER_PREPARE_ALLOW_NON_CALLBACKS, ASENSOR_STATUS_ACCURACY_HIGH, ASENSOR_STATUS_ACCURACY_LOW,
    ASENSOR_STATUS_ACCURACY_MEDIUM, ASENSOR_STATUS_NO_CONTACT, ASENSOR_STATUS_UNRELIABLE,
    ASENSOR_TYPE_ACCELEROMETER, ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR, ASENSOR_TYPE_GYROSCOPE,
};
use num_derive::FromPrimitive;
// use num_traits::FromPrimitive;
use std::mem::MaybeUninit;

#[derive(FromPrimitive)]
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
    Compass = ASENSOR_TYPE_GEOMAGNETIC_ROTATION_VECTOR as isize,
}

pub struct Sensor {
    sensor: *const ASensor,
}

unsafe impl Sync for Sensor {}
unsafe impl Send for Sensor {}

pub struct SensorManager {
    manager: *mut ASensorManager,
}

unsafe impl Sync for SensorManager {}
unsafe impl Send for SensorManager {}

#[derive(Debug)]
pub struct SensorEvent {
    accuracy: i8,
    sensor: SensorType,
    timestamp: i64,
    values: Vec<f32>,
}

#[derive(Debug)]
pub struct SensorEventQueue {
    queue: *mut ASensorEventQueue,
}

unsafe impl Sync for SensorEventQueue {}
unsafe impl Send for SensorEventQueue {}

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
            ASensorManager_createEventQueue(self.manager, looper_ptr, 0, None, std::ptr::null_mut())
        };
        assert!(!queue.is_null(), "*mut ASensorEventQueue is null");
        SensorEventQueue { queue }
    }

    pub fn destroy_event_queue(&self, queue: SensorEventQueue) {
        let status = unsafe { ASensorManager_destroyEventQueue(self.manager, queue.queue) };
        assert!(status >= 0);
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
        unsafe {
            // non-blocking
            ALooper_pollAll(0, &mut fd, &mut events, &mut data)
        };
        let mut events = Vec::new();
        let mut event: MaybeUninit<ASensorEvent> = MaybeUninit::uninit();
        let mut event_count =
            unsafe { ASensorEventQueue_getEvents(self.queue, event.as_mut_ptr(), 1) };
        let mut event = unsafe { event.assume_init() };
        while event_count > 0 {
            if let Some(sensor_type) = num::FromPrimitive::from_i32(event.reserved0.clone()) {
                match sensor_type {
                    SensorType::Accelerometer => {
                        events.push(SensorEvent {
                            accuracy: unsafe {
                                event.__bindgen_anon_1.__bindgen_anon_1.acceleration.status
                            },
                            sensor: SensorType::Accelerometer,
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
                    SensorType::Gyroscope => todo!(),
                    SensorType::Compass => todo!(),
                }
            }
            event_count =
                unsafe { ASensorEventQueue_getEvents(self.queue, &mut event as *mut _, 1) };
        }
        events
    }

    pub fn disable_sensor(&self, sensor: &Sensor) {
        unsafe { ASensorEventQueue_disableSensor(self.queue, sensor.sensor) };
    }
}

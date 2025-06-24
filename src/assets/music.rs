use std::time::Duration;

use kira::{
    sound::static_sound::{StaticSoundData, StaticSoundHandle},
    Easing, StartTime, Tween,
};
use pyo3::prelude::*;

use crate::math::audio::linear_to_db;

#[pyclass]
#[derive(Debug)]
pub struct Music {
    pub volume: f32,
    pub speed: f32,
    pub r#loop: bool,
    pub paused: bool,
    pub data: StaticSoundData,
    pub handle: Option<StaticSoundHandle>,
}

#[pymethods]
impl Music {
    #[getter]
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    #[setter]
    pub fn set_volume(&mut self, value: f32) {
        self.volume = value;
        self.data = self.data.volume(linear_to_db(value));

        if let Some(handle) = self.handle.as_mut() {
            handle.set_volume(
                linear_to_db(value),
                Tween {
                    start_time: StartTime::Immediate,
                    duration: Duration::from_secs(0),
                    easing: Easing::Linear,
                },
            );
        }
    }

    #[getter]
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    #[setter]
    pub fn set_speed(&mut self, value: f32) {
        self.speed = value;
        self.data = self.data.playback_rate(value as f64);

        if let Some(handle) = self.handle.as_mut() {
            handle.set_playback_rate(
                value as f64,
                Tween {
                    start_time: StartTime::Immediate,
                    duration: Duration::from_secs(0),
                    easing: Easing::Linear,
                },
            );
        }
    }

    #[getter]
    pub fn get_loop(&self) -> bool {
        self.r#loop
    }

    #[setter]
    pub fn set_loop(&mut self, value: bool) {
        self.r#loop = value;
        self.data = if value {
            self.data.loop_region(..)
        } else {
            self.data.loop_region(None)
        };

        if let Some(handle) = self.handle.as_mut() {
            if value {
                handle.set_loop_region(..);
            } else {
                handle.set_loop_region(None);
            }
        }
    }

    #[getter]
    pub fn get_paused(&self) -> bool {
        self.paused
    }

    #[setter]
    pub fn set_paused(&mut self, value: bool) {
        if let Some(handle) = self.handle.as_mut() {
            self.paused = value;

            if value {
                handle.pause(Tween {
                    start_time: StartTime::Immediate,
                    duration: Duration::from_secs(0),
                    easing: Easing::Linear,
                });
            } else {
                handle.resume(Tween {
                    start_time: StartTime::Immediate,
                    duration: Duration::from_secs(0),
                    easing: Easing::Linear,
                });
            }
        } else {
            self.paused = false;
        }
    }
}

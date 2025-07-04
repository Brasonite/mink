use kira::sound::static_sound::StaticSoundData;
use pyo3::prelude::*;

use crate::math::audio::linear_to_db;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Sound {
    pub volume: f32,
    pub speed: f32,
    pub data: StaticSoundData,
}

#[pymethods]
impl Sound {
    #[getter]
    pub fn get_volume(&self) -> f32 {
        self.volume
    }

    #[setter]
    pub fn set_volume(&mut self, value: f32) {
        self.volume = value;
        self.data = self.data.volume(linear_to_db(value));
    }

    #[getter]
    pub fn get_speed(&self) -> f32 {
        self.speed
    }

    #[setter]
    pub fn set_speed(&mut self, value: f32) {
        self.speed = value;
        self.data = self.data.playback_rate(value as f64);
    }
}

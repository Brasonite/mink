use pyo3::prelude::*;

pub mod api;

#[pyclass]
pub struct Time {
    pub delta: f32,
}

impl Time {
    pub fn new() -> Self {
        Self { delta: 0.0000001 }
    }
}

#[pymethods]
impl Time {
    pub fn delta(&self) -> f32 {
        self.delta
    }
}

use pyo3::prelude::*;

pub mod api;

#[pyclass]
pub struct Stats {}

impl Stats {
    pub fn new() -> Self {
        Self {}
    }
}

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    #[pyo3(get, set)]
    pub x: f32,
    #[pyo3(get, set)]
    pub y: f32,
}

impl Vec2 {
    pub fn into_glam(&self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

#[pymethods]
impl Vec2 {
    #[new]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn __str__(&self) -> String {
        self.to_string()
    }

    pub fn __repr__(&self) -> String {
        self.to_string()
    }
}

impl ToString for Vec2 {
    fn to_string(&self) -> String {
        format!("Vec2({:.2}, {:.2})", self.x, self.y)
    }
}

impl Into<glam::Vec2> for Vec2 {
    fn into(self) -> glam::Vec2 {
        glam::Vec2::new(self.x, self.y)
    }
}

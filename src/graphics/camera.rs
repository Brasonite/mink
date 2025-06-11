use pyo3::prelude::*;

use crate::math::vectors::Vec2;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Camera {
    #[pyo3(get, set)]
    pub size: Option<Vec2>,
    #[pyo3(get, set)]
    pub position: Vec2,
    #[pyo3(get, set)]
    pub rotation: f32,
    #[pyo3(get, set)]
    pub zoom: f32,
}

impl Camera {
    pub fn matrix(&self, viewport_size: [u32; 2]) -> glam::Mat4 {
        let (width, height) = match self.size {
            Some(size) => (size.x, size.y),
            None => (viewport_size[0] as f32, viewport_size[1] as f32),
        };

        let view = glam::Mat4::look_to_lh(
            glam::Vec3::new(self.position.x, self.position.y, -1.0),
            glam::Vec3::Z,
            glam::Vec3::Y,
        );

        let projection = glam::Mat4::orthographic_lh(
            -width / 2.0,
            width / 2.0,
            -height / 2.0,
            height / 2.0,
            0.001,
            1000.0,
        );

        projection * view
    }
}

#[pymethods]
impl Camera {
    #[new]
    pub const fn new() -> Self {
        Self {
            size: None,
            position: Vec2::new(0.0, 0.0),
            rotation: 0.0,
            zoom: 1.0,
        }
    }
}

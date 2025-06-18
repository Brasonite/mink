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

        let (width, height) = {
            let zoom_multiplier = 1.0 / self.zoom.max(0.00000001);
            (width * zoom_multiplier, height * zoom_multiplier)
        };

        let view = glam::Mat4::from_rotation_z(self.rotation)
            * glam::Mat4::look_to_lh(
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

    pub fn world_to_screen_transform(&self, viewport_size: glam::Vec2) -> glam::Affine2 {
        let (width, height) = match self.size {
            Some(size) => (size.x, size.y),
            None => (viewport_size.x, viewport_size.y),
        };

        let (width, height) = {
            let zoom_multiplier = 1.0 / self.zoom.max(0.00000001);
            (width * zoom_multiplier, height * zoom_multiplier)
        };

        let scale = glam::Vec2::new(viewport_size.x / width, -viewport_size.y / height);

        glam::Affine2::from_translation(viewport_size / 2.0)
            * glam::Affine2::from_scale(scale)
            * glam::Affine2::from_translation(-self.position.into_glam())
            * glam::Affine2::from_angle(self.rotation)
    }

    pub fn screen_to_world_transform(&self, viewport_size: glam::Vec2) -> glam::Affine2 {
        self.world_to_screen_transform(viewport_size).inverse()
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

    pub fn project(&self, position: Vec2, window_size: Vec2) -> Vec2 {
        self.screen_to_world_transform(window_size.into_glam())
            .transform_point2(position.into_glam())
            .into()
    }

    pub fn unproject(&self, position: Vec2, window_size: Vec2) -> Vec2 {
        self.world_to_screen_transform(window_size.into_glam())
            .transform_point2(position.into_glam())
            .into()
    }
}

use pyo3::prelude::*;

use crate::math::vectors::Vec2;

#[pyclass]
#[derive(Debug)]
pub struct Camera {
    #[pyo3(get, set)]
    pub size: Option<Py<Vec2>>,
    #[pyo3(get, set)]
    pub position: Py<Vec2>,
    #[pyo3(get, set)]
    pub rotation: f32,
    #[pyo3(get, set)]
    pub zoom: f32,
}

impl Camera {
    pub fn build_matrix(
        size: glam::Vec2,
        position: glam::Vec2,
        rotation: f32,
        zoom: f32,
    ) -> glam::Mat4 {
        let view = glam::Mat4::from_rotation_z(rotation)
            * glam::Mat4::look_to_lh(
                glam::Vec3::new(position.x, position.y, -1.0),
                glam::Vec3::Z,
                glam::Vec3::Y,
            );

        let size = size * (1.0 / zoom.max(0.00000001));

        let projection = glam::Mat4::orthographic_lh(
            -size.x / 2.0,
            size.x / 2.0,
            -size.y / 2.0,
            size.y / 2.0,
            0.001,
            1000.0,
        );

        projection * view
    }

    pub fn matrix<'a>(&self, py: Python<'a>, viewport_size: glam::Vec2) -> glam::Mat4 {
        let size = match &self.size {
            Some(size) => size.borrow(py).0,
            None => viewport_size,
        };

        let position = self.position.borrow(py).0;

        Self::build_matrix(size, position, self.rotation, self.zoom)
    }

    pub fn world_to_screen_transform<'a>(
        &self,
        py: Python<'a>,
        viewport_size: glam::Vec2,
    ) -> glam::Affine2 {
        let (width, height) = match &self.size {
            Some(size) => {
                let size = size.borrow(py).clone();
                (size.x, size.y)
            }
            None => (viewport_size.x, viewport_size.y),
        };

        let (width, height) = {
            let zoom_multiplier = 1.0 / self.zoom.max(0.00000001);
            (width * zoom_multiplier, height * zoom_multiplier)
        };

        let scale = glam::Vec2::new(viewport_size.x / width, -viewport_size.y / height);

        let position = self.position.borrow(py).clone();
        glam::Affine2::from_translation(viewport_size / 2.0)
            * glam::Affine2::from_scale(scale)
            * glam::Affine2::from_angle(self.rotation)
            * glam::Affine2::from_translation(-*position)
    }

    pub fn screen_to_world_transform<'a>(
        &self,
        py: Python<'a>,
        viewport_size: glam::Vec2,
    ) -> glam::Affine2 {
        self.world_to_screen_transform(py, viewport_size).inverse()
    }
}

#[pymethods]
impl Camera {
    #[new]
    pub fn new<'a>(py: Python<'a>) -> Self {
        Self {
            size: None,
            position: Vec2::ZERO
                .into_pyobject(py)
                .map(|x| x.unbind())
                .expect("Failed to create camera position vector"),
            rotation: 0.0,
            zoom: 1.0,
        }
    }

    pub fn project<'a>(&self, py: Python<'a>, position: Vec2, window_size: Vec2) -> Vec2 {
        self.screen_to_world_transform(py, *window_size)
            .transform_point2(*position)
            .into()
    }

    pub fn unproject<'a>(&self, py: Python<'a>, position: Vec2, window_size: Vec2) -> Vec2 {
        self.world_to_screen_transform(py, *window_size)
            .transform_point2(*position)
            .into()
    }
}

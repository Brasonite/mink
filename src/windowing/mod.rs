use std::sync::Arc;

use pyo3::prelude::*;
use winit::{dpi::PhysicalSize, window::Window as WinitWindow};

use crate::math::vectors::Vec2;

pub mod api;

#[pyclass]
pub struct Window {
    window: Arc<WinitWindow>,
}

impl Window {
    pub fn new(window: Arc<WinitWindow>) -> Self {
        Self { window }
    }
}

#[pymethods]
impl Window {
    pub fn resizable(&self) -> bool {
        self.window.is_resizable()
    }

    pub fn set_resizable(&self, value: bool) {
        self.window.set_resizable(value);
    }

    pub fn size(&self) -> Vec2 {
        let size = self.window.inner_size();
        Vec2::new(size.width as f32, size.height as f32)
    }

    pub fn set_size(&self, value: &Vec2) {
        let _ = self
            .window
            .request_inner_size(PhysicalSize::new(value.x as u32, value.y as u32));
    }

    pub fn title(&self) -> String {
        self.window.title()
    }

    pub fn set_title(&self, title: &str) {
        self.window.set_title(title);
    }
}

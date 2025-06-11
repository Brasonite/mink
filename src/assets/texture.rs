use std::sync::Arc;

use pyo3::prelude::*;

#[pyclass]
#[derive(Debug, Clone)]
pub struct Texture {
    pub path: Arc<String>,
    pub texture: Arc<wgpu::Texture>,
    pub view: Arc<wgpu::TextureView>,
    pub binding: Arc<wgpu::BindGroup>,
    pub size: Arc<glam::Vec2>,
}

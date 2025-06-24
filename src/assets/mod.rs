use std::sync::Arc;

use kira::sound::static_sound::StaticSoundData;
use pyo3::prelude::*;

use crate::{
    assets::{music::Music, sound::Sound, texture::Texture},
    graphics::{builtin::VideoBuiltins, stack::VideoStack},
};

pub mod api;
pub mod music;
pub mod sound;
pub mod texture;

#[pyclass]
pub struct Assets {
    pub device: Arc<wgpu::Device>,
    pub queue: Arc<wgpu::Queue>,
    pub builtins: Arc<VideoBuiltins>,
    pub root: String,
}

impl Assets {
    pub fn new(video: &VideoStack, builtins: Arc<VideoBuiltins>) -> Self {
        Self {
            device: Arc::clone(&video.device),
            queue: Arc::clone(&video.queue),
            builtins,
            root: "assets".to_string(),
        }
    }
}

#[pymethods]
impl Assets {
    pub fn set_root(&mut self, path: &str) {
        self.root = path.to_string();
    }

    pub fn resolve_path(&self, path: &str) -> String {
        format!("{}/{}", self.root, path)
    }

    pub fn music(&self, path: &str) -> Music {
        let filepath = self.resolve_path(path);

        Music {
            volume: 1.0,
            speed: 1.0,
            r#loop: false,
            paused: false,
            data: StaticSoundData::from_file(filepath).expect("Failed to load music"),
            handle: None,
        }
    }

    pub fn sound(&self, path: &str) -> Sound {
        let filepath = self.resolve_path(path);

        Sound {
            volume: 1.0,
            speed: 1.0,
            data: StaticSoundData::from_file(filepath).expect("Failed to load sound"),
        }
    }

    pub fn texture(&self, path: &str) -> Texture {
        let filepath = self.resolve_path(path);

        let image = image::open(filepath).expect("Failed to load image");
        let rgba = image.to_rgba8();

        let size = wgpu::Extent3d {
            width: image.width(),
            height: image.height(),
            depth_or_array_layers: 1,
        };

        let texture = self.device.create_texture(&wgpu::TextureDescriptor {
            label: Some(&format!("Texture: {path}")),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8Unorm,
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
            view_formats: &[],
        });

        self.queue.write_texture(
            wgpu::TexelCopyTextureInfo {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            &rgba,
            wgpu::TexelCopyBufferLayout {
                offset: 0,
                bytes_per_row: Some(4 * size.width),
                rows_per_image: Some(size.height),
            },
            size,
        );

        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let binding = self.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some(&format!("Texture binding: {path}")),
            layout: &self.builtins.layouts.texture,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.builtins.sampler),
                },
            ],
        });

        Texture {
            path: Arc::new(path.to_string()),
            texture: Arc::new(texture),
            view: Arc::new(view),
            binding: Arc::new(binding),
            size: Arc::new([size.width as f32, size.height as f32].into()),
        }
    }
}

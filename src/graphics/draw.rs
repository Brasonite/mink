use std::{collections::HashMap, sync::Arc};

use pyo3::prelude::*;

use crate::{
    assets::texture::Texture,
    graphics::{builtin::VideoBuiltins, camera::Camera, quad::Quad, stack::VideoStack},
    math::{colors::Color, matrices::model_matrix, vectors::Vec2},
};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct RawInstance {
    pub matrix: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl RawInstance {
    pub const fn buffer_layout() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 4]>() as wgpu::BufferAddress,
                    shader_location: 3,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 8]>() as wgpu::BufferAddress,
                    shader_location: 4,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 12]>() as wgpu::BufferAddress,
                    shader_location: 5,
                    format: wgpu::VertexFormat::Float32x4,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 16]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DrawInstance {
    pub camera: Camera,
    pub model: glam::Mat4,
    pub color: Color,
}

impl DrawInstance {
    pub fn into_raw(self, viewport_size: [u32; 2]) -> RawInstance {
        RawInstance {
            matrix: (self.camera.matrix(viewport_size) * self.model).to_cols_array_2d(),
            color: self.color.as_array(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum DrawAttachment {
    Pipeline(Arc<wgpu::RenderPipeline>),
    Texture(Arc<wgpu::BindGroup>),
}

impl DrawAttachment {
    pub fn attach(&self, pass: &mut wgpu::RenderPass) {
        match self {
            Self::Pipeline(pipeline) => pass.set_pipeline(pipeline),
            Self::Texture(binding) => pass.set_bind_group(0, binding.as_ref(), &[]),
        }
    }
}

pub struct DrawBatch {
    pub label: String,
    pub attachments: Vec<DrawAttachment>,
    pub instances: Vec<DrawInstance>,
    pub buffer: wgpu::Buffer,
    pub size: wgpu::BufferAddress,
    pub count: u32,
    pub lifetime: usize,
}

impl DrawBatch {
    pub const LIFETIME: usize = 10;

    pub fn new(
        device: &wgpu::Device,
        label: &str,
        attachments: Vec<DrawAttachment>,
        instances: Vec<DrawInstance>,
    ) -> Self {
        let count = instances.len() as u32;
        let size = std::mem::size_of::<RawInstance>() as wgpu::BufferAddress
            * count as wgpu::BufferAddress;

        let buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(label),
            size,
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        Self {
            label: label.to_string(),
            attachments,
            instances,
            buffer,
            size,
            count,
            lifetime: Self::LIFETIME,
        }
    }

    pub fn add(&mut self, instance: DrawInstance) {
        self.instances.push(instance);
        self.count += 1;
    }

    pub fn write(&mut self, video: &VideoStack) -> u32 {
        let size = std::mem::size_of::<RawInstance>() as wgpu::BufferAddress
            * self.count as wgpu::BufferAddress;
        if size > self.size {
            self.buffer = video.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&self.label),
                size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        if size > 0 {
            let viewport_size = [video.config.width, video.config.height];
            let instances: Vec<RawInstance> = self
                .instances
                .drain(..)
                .map(|x| x.into_raw(viewport_size))
                .collect();

            video
                .queue
                .write_buffer(&self.buffer, 0, bytemuck::cast_slice(&instances));
        }

        if self.count > 0 {
            self.lifetime = Self::LIFETIME;
        } else if self.lifetime > 0 {
            self.lifetime -= 1;
        }

        let count = self.count;
        self.count = 0;

        count
    }
}

pub struct Batcher {
    pub batches: HashMap<String, DrawBatch>,
}

impl Batcher {
    pub fn new() -> Self {
        Self {
            batches: HashMap::new(),
        }
    }

    pub fn add(
        &mut self,
        device: &wgpu::Device,
        id: &str,
        attachments: &[DrawAttachment],
        instance: DrawInstance,
    ) {
        match self.batches.get_mut(id) {
            Some(batch) => batch.add(instance),
            None => {
                let batch = DrawBatch::new(device, id, attachments.to_vec(), vec![instance]);

                self.batches.insert(id.to_string(), batch);
            }
        }
    }

    pub fn cleanup(&mut self) {
        let mut dead_batches: Vec<String> = Vec::new();
        for id in self.batches.keys() {
            if self.batches[id].lifetime == 0 {
                dead_batches.push(id.clone());
            }
        }

        for id in dead_batches {
            self.batches.remove(&id);
        }
    }
}

#[pyclass]
pub struct Draw {
    pub device: Arc<wgpu::Device>,
    pub builtins: Arc<VideoBuiltins>,
    pub quad: Quad,
    pub default_camera: Camera,
    pub current_camera: Option<Camera>,
    pub batcher: Batcher,
}

impl Draw {
    pub fn new(video: &VideoStack, builtins: Arc<VideoBuiltins>) -> Self {
        let quad = Quad::new(video);

        Self {
            device: Arc::clone(&video.device),
            builtins,
            quad,
            default_camera: Camera::new(),
            current_camera: None,
            batcher: Batcher::new(),
        }
    }

    pub fn submit(&mut self, video: &VideoStack, pass: &mut wgpu::RenderPass) {
        self.quad.apply(pass);

        for batch in self.batcher.batches.values_mut() {
            let count = batch.write(video);

            if count == 0 {
                continue;
            }

            for attachment in &batch.attachments {
                attachment.attach(pass);
            }

            pass.set_vertex_buffer(1, batch.buffer.slice(..));
            pass.draw_indexed(0..Quad::NUM_INDICES, 0, 0..count);
        }

        self.batcher.cleanup();
    }
}

#[pymethods]
impl Draw {
    pub fn set_camera(&mut self, camera: Option<&Camera>) {
        self.current_camera = camera.map(|x| x.clone());
    }

    pub fn sprite(
        &mut self,
        texture: &Texture,
        position: &Vec2,
        rotation: Option<f32>,
        scale: Option<Vec2>,
        tint: Option<Color>,
    ) {
        self.batcher.add(
            &self.device,
            &texture.path,
            &[
                DrawAttachment::Pipeline(Arc::clone(&self.builtins.pipelines.sprite.pipeline)),
                DrawAttachment::Texture(Arc::clone(&texture.binding)),
            ],
            DrawInstance {
                camera: self
                    .current_camera
                    .clone()
                    .unwrap_or(self.default_camera.clone()),
                model: model_matrix(
                    &position,
                    rotation.unwrap_or(0.0),
                    &(*texture.size * scale.map(|x| x.into()).unwrap_or(glam::Vec2::ONE)),
                ),
                color: tint.unwrap_or(Color::WHITE),
            },
        );
    }
}

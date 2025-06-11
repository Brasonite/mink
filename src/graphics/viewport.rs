use glam::{Mat4, Vec2, Vec3};

use crate::graphics::{builtin::VideoBuiltins, stack::VideoStack};

pub struct Viewport {
    pub position: Vec2,
    pub size: Vec2,
    pub buffer: wgpu::Buffer,
    pub binding: wgpu::BindGroup,
}

impl Viewport {
    pub fn new(video: &VideoStack, builtins: &VideoBuiltins) -> Self {
        let buffer = video.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Viewport buffer"),
            size: std::mem::size_of::<[[f32; 4]; 4]>() as wgpu::BufferAddress,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let binding = video.device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &builtins.layouts.camera,
            label: Some("Viewport bind group"),
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: buffer.as_entire_binding(),
            }],
        });

        Self {
            position: Vec2::ZERO,
            size: [video.config.width as f32, video.config.height as f32].into(),
            buffer,
            binding,
        }
    }

    pub fn update(&mut self, video: &VideoStack) {
        self.size = [video.config.width as f32, video.config.height as f32].into();

        video.queue.write_buffer(
            &self.buffer,
            0,
            bytemuck::cast_slice(&Self::matrix(self.position, self.size).to_cols_array()),
        );
    }

    pub fn apply(&self, pass: &mut wgpu::RenderPass) {
        pass.set_bind_group(0, &self.binding, &[]);
    }

    pub fn matrix(position: Vec2, size: Vec2) -> Mat4 {
        let view = Mat4::look_to_lh(Vec3::new(position.x, position.y, -1.0), Vec3::Z, Vec3::Y);
        let projection = Mat4::orthographic_lh(0.0, size.x, size.y, 0.0, 0.001, 1000.0);

        projection * view
    }
}

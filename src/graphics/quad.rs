use wgpu::util::DeviceExt;

use crate::{graphics::stack::VideoStack, math::vertex::Vertex};

pub struct Quad {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
}

impl Quad {
    pub const NUM_INDICES: u32 = 6;

    pub fn new(video: &VideoStack) -> Self {
        let vertex_buffer = video
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad vertex buffer"),
                contents: bytemuck::cast_slice(&[
                    Vertex::new([-0.5, -0.5], [0.0, 0.0]),
                    Vertex::new([0.5, -0.5], [1.0, 0.0]),
                    Vertex::new([0.5, 0.5], [1.0, 1.0]),
                    Vertex::new([-0.5, 0.5], [0.0, 1.0]),
                ]),
                usage: wgpu::BufferUsages::VERTEX,
            });

        let index_buffer = video
            .device
            .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Quad index buffer"),
                contents: bytemuck::cast_slice(&[2u16, 0, 3, 2, 1, 0]),
                usage: wgpu::BufferUsages::INDEX,
            });

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn apply(&self, pass: &mut wgpu::RenderPass) {
        pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
    }
}

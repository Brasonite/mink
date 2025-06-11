use std::sync::Arc;

use crate::{
    graphics::{
        draw::DrawInstance, pipeline::GraphicsPipeline, stack::VideoStack, target::RenderTarget,
    },
    math::vertex::Vertex,
};

pub struct BuiltinLayouts {
    pub texture: Arc<wgpu::BindGroupLayout>,
    pub camera: Arc<wgpu::BindGroupLayout>,
}

impl BuiltinLayouts {
    pub fn new(video: &VideoStack) -> Self {
        let texture = video
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Texture bind group layout"),
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                            view_dimension: wgpu::TextureViewDimension::D2,
                            multisampled: false,
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
            });

        let camera = video
            .device
            .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                label: Some("Camera bind group layout"),
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
            });

        Self {
            texture: Arc::new(texture),
            camera: Arc::new(camera),
        }
    }
}

pub struct BuiltinPipelines {
    pub sprite: Arc<GraphicsPipeline>,
}

impl BuiltinPipelines {
    pub fn new(video: &VideoStack, layouts: &BuiltinLayouts) -> Self {
        let sprite = {
            let shader = video
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("Sprite shader"),
                    source: wgpu::ShaderSource::Wgsl(mink_shaders::SPRITE.into()),
                });

            let layout = video
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Sprite pipeline layout"),
                    bind_group_layouts: &[&layouts.camera, &layouts.texture],
                    push_constant_ranges: &[],
                });

            let pipeline = video
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("Sprite pipeline"),
                    layout: Some(&layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        buffers: &[Vertex::buffer_layout(), DrawInstance::buffer_layout()],
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: &shader,
                        entry_point: Some("fs_main"),
                        compilation_options: wgpu::PipelineCompilationOptions::default(),
                        targets: &[Some(wgpu::ColorTargetState {
                            format: video.config.format,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::all(),
                        })],
                    }),
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Ccw,
                        cull_mode: None,
                        polygon_mode: wgpu::PolygonMode::Fill,
                        unclipped_depth: false,
                        conservative: false,
                    },
                    depth_stencil: None,
                    multisample: wgpu::MultisampleState {
                        count: RenderTarget::SAMPLE_COUNT,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    multiview: None,
                    cache: None,
                });

            GraphicsPipeline {
                layout: Arc::new(layout),
                pipeline: Arc::new(pipeline),
            }
        };

        Self {
            sprite: Arc::new(sprite),
        }
    }
}

pub struct VideoBuiltins {
    pub layouts: BuiltinLayouts,
    pub pipelines: BuiltinPipelines,
    pub sampler: wgpu::Sampler,
}

impl VideoBuiltins {
    pub fn new(video: &VideoStack) -> Self {
        let layouts = BuiltinLayouts::new(video);
        let pipelines = BuiltinPipelines::new(video, &layouts);

        let sampler = video.device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Linear,
            ..Default::default()
        });

        Self {
            layouts,
            pipelines,
            sampler,
        }
    }
}

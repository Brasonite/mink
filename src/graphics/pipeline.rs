use std::sync::Arc;

pub struct GraphicsPipeline {
    pub layout: Arc<wgpu::PipelineLayout>,
    pub pipeline: Arc<wgpu::RenderPipeline>,
}

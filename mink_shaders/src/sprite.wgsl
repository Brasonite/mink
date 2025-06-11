struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) uv: vec2<f32>,
}

struct InstanceInput {
    @location(2) matrix_1: vec4<f32>,
    @location(3) matrix_2: vec4<f32>,
    @location(4) matrix_3: vec4<f32>,
    @location(5) matrix_4: vec4<f32>,
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv: vec2<f32>,
}

@group(0) @binding(0)
var texture: texture_2d<f32>;
@group(0) @binding(1)
var texture_sampler: sampler;

@vertex
fn vs_main(vertex: VertexInput, instance: InstanceInput) -> VertexOutput {
    let matrix = mat4x4<f32>(
        instance.matrix_1,
        instance.matrix_2,
        instance.matrix_3,
        instance.matrix_4,
    );

    var out: VertexOutput;
    out.clip_position = matrix * vec4<f32>(vertex.position, 0.0, 1.0) ;
    out.uv = vertex.uv;

    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, in.uv);
}
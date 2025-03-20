#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput

// struct VertexOutput {
    // @builtin(position) position: vec4<f32>,
    // @location(0) uv: vec2<f32>,
// }

struct Uniforms {
    time: f32,
    _webgl2_padding: vec3<f32>,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@fragment
fn init(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let fact = (sin(uniforms.time * 0.1) + 1) / 2;
    let color = fact * vec3<f32>(1.0, 0.0, 0.0) + (1 - fact) * vec3<f32>(0.0, 0.0, 1.0);
    return vec4<f32>(color, 1.0);
}

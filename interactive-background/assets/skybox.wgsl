#import bevy_core_pipeline::fullscreen_vertex_shader::FullscreenVertexOutput
#import noisy_bevy::simplex_noise_2d

// struct VertexOutput {
    // @builtin(position) position: vec4<f32>,
    // @location(0) uv: vec2<f32>,
// }

struct Uniforms {
    time: f32,
    _webgl2_padding: vec3<f32>,
}
@group(0) @binding(0) var<uniform> uniforms: Uniforms;

const SIN_PERIOD: f32 = 1.5;
const SCALE: f32 = 1000.0;
const THRESH: f32 = 0.85;

fn fact(x: f32) -> f32 {
    return sin(SIN_PERIOD * x)/2 + 0.5;
}

@fragment
fn init(in: FullscreenVertexOutput) -> @location(0) vec4<f32> {
    let noisy_text_pos = in.uv * SCALE;
    let noisy_star = simplex_noise_2d(noisy_text_pos);
    let noisy_shift = simplex_noise_2d(noisy_text_pos + SCALE) * radians(180.0);
    let noisy_intensity = simplex_noise_2d(noisy_text_pos - SCALE);

    if (noisy_star < THRESH) {
        return vec4<f32>(vec3<f32>(0.0), 1.0);
    }

    let intensity = fact(uniforms.time + noisy_shift);
    let value = noisy_star * noisy_intensity * intensity;
    return vec4<f32>(vec3<f32>(value), 1.0);
}

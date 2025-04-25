struct Uniforms {
    center: vec2<f32>,
    radius: f32,
    color: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

// Define the value of PI
const PI: f32 = 3.141592653589793;

@vertex
fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
    let segment_count = 64.0; // Adjust for smoother/rougher circle
    let angle = f32(vertex_index) / segment_count * 2.0 * PI;
    let x = cos(angle) * uniforms.radius + uniforms.center.x;
    let y = sin(angle) * uniforms.radius + uniforms.center.y;

    var out: VertexOutput;
    out.position = vec4<f32>(x, y, 0.0, 1.0);
    out.uv = vec2<f32>(cos(angle) * 0.5 + 0.5, sin(angle) * 0.5 + 0.5); // Optional UVs
    return out;
}

@fragment
fn fs_main(@location(0) uv: vec2<f32>) -> @location(0) vec4<f32> {
    // Method 1: Simple distance check
    let dist = distance(uv, vec2<f32>(0.5));
    if (dist < 0.5) {
        return uniforms.color;
    } else {
        discard;
    }

    // Method 2: Using UVs for a smoother boundary (optional)
    // let center_to_frag = uv - vec2<f32>(0.5);
    // if (dot(center_to_frag, center_to_frag) < 0.25) {
    //     return uniforms.color;
    // } else {
    //     discard;
    // }
}
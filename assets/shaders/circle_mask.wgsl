// Define the output from the vertex shader
struct VertexOutput {
    @builtin(position) position: vec4<f32>, // The position in clip space
    @location(0) uv: vec2<f32>,            // The UV coordinates
};

// Define the material as a uniform buffer
@group(0) @binding(0)
var<uniform> material: CircleMaskMaterial;

struct CircleMaskMaterial {
    color: vec4<f32>,
    radius: f32,
    screen_size: vec2<f32>,
    enabled: f32,
};

// Fragment shader
@fragment
fn fragment(mesh: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0); 
}
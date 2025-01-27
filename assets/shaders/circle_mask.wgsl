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
    if (material.enabled < 0.5) {
        return vec4<f32>(1.0, 0.0, 1.0, 1); // Transparent if mask is disabled
    }

    let center = material.screen_size * 0.5; // Center of the screen
    let coord = mesh.uv * material.screen_size; // Convert UV to screen coordinates
    let dist = distance(coord, center); // Distance from the center

    if (dist < material.radius) {
        return material.color; // Render the circle
    }

    return vec4<f32>(0.0, 0.0, 0.0, 1.0); // Transparent outside the circle
}
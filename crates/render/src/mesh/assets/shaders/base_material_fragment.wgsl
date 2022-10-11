@group(0)
@binding(0)
var<uniform> r_color: vec3<f32>;

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4(r_color, 1.0);
}
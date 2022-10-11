struct VertexOutput {
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vertex(@location(0) pos: vec2<f32>) -> @builtin(position) vec4<f32> {
    return vec4<f32>(pos, 0.0, 1.0);
}

@group(0)
@binding(0)
var<uniform> color: vec3<f32>;

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(color, 1.0);
}
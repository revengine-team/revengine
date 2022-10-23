struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(1)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@group(2)
@binding(0)
var<uniform> view: mat4x4<f32>;

@group(2)
@binding(1)
var<uniform> projection: mat4x4<f32>;

@vertex
fn vertex(
    @location(0) position: vec3<f32>,
    @location(1) tex_coord: vec2<f32>,
    @location(2) normal: vec3<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coord;
    result.position = projection * view * transform * vec4(position, 1.0);
    result.world_pos = transform * vec4(position, 1.0);
    result.normal = (transform * vec4(normal, 1.0)).xyz;
    return result;
}
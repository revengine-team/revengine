@group(0)
@binding(0)
var<uniform> r_color: vec3<f32>;

@group(0)
@binding(1)
var t_diffuse: texture_2d<f32>;

@group(0)
@binding(2)
var s_diffuse: sampler;

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    let texture_color = textureSample(t_diffuse, s_diffuse, vertex.tex_coord);
    return vec4(r_color, 1.0) * texture_color;
}
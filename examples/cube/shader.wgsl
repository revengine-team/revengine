struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@group(1)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@vertex
fn vertex(
    @location(0) position: vec4<f32>,
    @location(1) tex_coord: vec2<f32>,
) -> VertexOutput {
    var result: VertexOutput;
    result.tex_coord = tex_coord;
    result.position = transform * position;
    return result;
}

@group(0)
@binding(0)
var r_color: texture_2d<f32>;
@group(0)
@binding(1)
var s_diffuse: sampler;

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {
    //let tex = textureLoad(r_color, vec2<i32>(vertex.tex_coord * 256.0), 0);
    let tex = textureSample(r_color, s_diffuse, vertex.tex_coord );
    //let v = f32(tex.x) / 255.0;
    //return vec4<f32>(1.0 - (v * 5.0), 1.0 - (v * 15.0), 1.0 - (v * 50.0), 1.0);
    return tex;
}

@fragment
fn fs_wire(vertex: VertexOutput) -> @location(0) vec4<f32> {
    return vec4<f32>(0.0, 0.5, 0.0, 0.5);
}
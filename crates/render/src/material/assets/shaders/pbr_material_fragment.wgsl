struct SpotLight {
    position: vec4<f32>,
    direction: vec4<f32>,
    diffuse: vec4<f32>,
    specular: vec4<f32>,
    cut_off: f32,
    outer_cut_off: f32,
    constant: f32,
    linear: f32,
    quadratic: f32,
};

struct DirLight {
    direction: vec4<f32>,
    diffuse: vec4<f32>,
    specular: vec4<f32>,
    strength: f32,
};

struct PointLight {
    position: vec4<f32>,
    diffuse: vec4<f32>,
    specular: vec4<f32>,
    constant: f32,
    linear: f32,
    quadratic: f32,
};

// PBR
@group(0)
@binding(0)
var<uniform> base_color: vec4<f32>;

@group(0)
@binding(1)
var<uniform> emissive_color: vec4<f32>;

@group(0)
@binding(2)
var<uniform> roughness: f32;

@group(0)
@binding(3)
var<uniform> metallic: f32;

@group(0)
@binding(4)
var texture: texture_2d<f32>;

@group(0)
@binding(5)
var texture_sampler: sampler;

@group(0)
@binding(6)
var noraml_texture: texture_2d<f32>;

@group(0)
@binding(7)
var normal_sampler: sampler;

// Lights
@group(3)
@binding(0)
var<uniform> dir_light: array<DirLight, 2>;

@group(3)
@binding(1)
var<uniform> spot_light: array<SpotLight, 8>;

@group(3)
@binding(2)
var<uniform> point_light: array<PointLight, 16>;

// Matrices
@group(1)
@binding(0)
var<uniform> transform: mat4x4<f32>;

@group(2)
@binding(0)
var<uniform> view: mat4x4<f32>;

@group(2)
@binding(1)
var<uniform> projection: mat4x4<f32>;

// Camera position
@group(2)
@binding(2)
var<uniform> camera_pos: vec3<f32>;

struct VertexOutput {
    @location(0) tex_coord: vec2<f32>,
    @location(1) normal: vec3<f32>,
    @location(2) world_pos: vec4<f32>,
    @builtin(position) position: vec4<f32>,
};

fn fresnel_schlick(theta: f32, F0: vec3<f32>) -> vec3<f32> {
    return F0 + (1.0 - F0) * pow(clamp(1.0 - theta, 0.0, 1.0), 5.0);
}  

let PI = 3.14159;
fn distribution_GGX(N: vec3<f32>, H: vec3<f32>, roughness: f32) -> f32 {
    let  a = roughness * roughness;
    let  a2 = a * a;
    let  NdotH = max(dot(N, H), 0.0);
    let  NdotH2 = NdotH * NdotH;

    let  num = a2;
    var denom = (NdotH2 * (a2 - 1.0) + 1.0);
    denom = PI * denom * denom;

    return num / denom;
}

fn geometry_GGX(NdotV: f32, roughness: f32) -> f32 {
    let r = (roughness + 1.0);
    let k = (r * r) / 8.0;

    let num = NdotV;
    let denom = NdotV * (1.0 - k) + k;

    return num / denom;
}

fn geometry_smith(N: vec3<f32>, V: vec3<f32>, L: vec3<f32>, roughness: f32) -> f32 {
    let  NdotV = max(dot(N, V), 0.0);
    let  NdotL = max(dot(N, L), 0.0);
    let  ggx2 = geometry_GGX(NdotV, roughness);
    let  ggx1 = geometry_GGX(NdotL, roughness);

    return ggx1 * ggx2;
}

@fragment
fn fragment(vertex: VertexOutput) -> @location(0) vec4<f32> {

    let N = normalize(vertex.normal);
    let V = normalize(camera_pos - vertex.world_pos.xyz);

    let texture_color = textureSample(texture, texture_sampler, vertex.tex_coord);
    let albedo = (base_color * texture_color).xyz;

    var F0 = vec3<f32>(0.04);
    F0 = mix(F0, albedo, metallic);

    var Lo = vec3<f32>(0.0);

    for (var i = 0; i < 16; i++) {
        let L = normalize(point_light[i].position.xyz - vertex.world_pos.xyz);
        let H = normalize(V + L);

        let distance = length(point_light[i].position.xyz - vertex.world_pos.xyz);
        let attenuation = 1.0 / (distance * distance);
        let radiance = point_light[i].diffuse.xyz * attenuation;

        // cook-torrance brdf
        let NDF = distribution_GGX(N, H, roughness);
        let G = geometry_smith(N, V, L, roughness);
        let F = fresnel_schlick(max(dot(H, V), 0.0), F0);

        let kS = F;
        var kD = vec3<f32>(1.0) - kS;
        kD *= 1.0 - metallic;	

        let numerator    = NDF * G * F;
        let denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0) + 0.0001;
        let specular     = numerator / denominator;  

        // add to outgoing radiance Lo
        let NdotL = max(dot(N, L), 0.0);                
        Lo += (kD * albedo / PI + specular) * radiance * NdotL; 
    }

    let ambient = vec3<f32>(0.03) * albedo * 0.3;
    var color = ambient + Lo;

    color = color / (color + vec3(1.0));
    color = pow(color, vec3<f32>(1.0/2.2)); 

    return vec4<f32>(color, 1.0);
}
[[location(0)]]
var<in> in_position: vec3<f32>;
[[location(1)]]
var<in> in_normal: vec3<f32>;
[[location(2)]]
var<in> in_color: vec3<f32>;

[[location(0)]]
var<out> out_color: vec3<f32>;
[[location(1)]]
var<out> out_normal: vec3<f32>;

[[builtin(position)]]
var<out> out_position: vec4<f32>;

[[block]]
struct Uniforms {
    model: mat4x4<f32>;
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
};

[[group(0), binding(0)]]
var<uniform> global: Uniforms;

[[stage(vertex)]]
fn main() {
    out_normal = mat3<f32>(transpose(inverse(global.view * global.model)) * in_normal;
    out_color = in_color;
    out_position = global.proj * global.view * global.model * vec4<f32>(in_position, 1.0);
}

[[location(0)]]
var<in> in_color: vec3<f32>;
[[location(1)]]
var<in> in_normal: vec3<f32>;
[[location(0)]]
var<out> out_color: vec4<f32>;

[[group(0), binding(0)]]
var<uniform> global: Uniforms;

[[stage(fragment)]
fn main() {
    const light: vec3<f32> = vec3(1.0, 1.0, -1.0);
    const view_light = mat3(view) * light;
    const lum = max(dot(normalize(in_normal), normalize(view_light)), 0.0);
    out_color = vec4<f32>(in_color * (0.2 + 0.8 * lum), 1.0);
}





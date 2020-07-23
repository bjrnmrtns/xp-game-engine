#version 450

layout(location=0) flat in vec3 in_color;
layout(location=1) flat in vec3 in_normal;
layout(location=0) out vec4 out_color;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

const vec3 light = vec3(1.0, 1.0, -1.0);

void main() {
    vec3 view_light = mat3(view) * light;
    float lum = max(dot(normalize(in_normal), normalize(view_light)), 0.0);
    out_color = vec4(in_color * (0.2 + 0.8 * lum), 1.0);
}
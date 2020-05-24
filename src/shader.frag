#version 450

layout(location=0) flat in vec3 in_color;
layout(location=1) flat in vec3 in_normal;
layout(location=0) out vec4 out_color;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

void main() {
    vec3 light = mat3(view) * vec3(0.0, 1.0, 0.0);
    float intensity = max(dot(in_normal, light), 0.01);
    out_color = vec4(in_color * intensity, 1.0);
}
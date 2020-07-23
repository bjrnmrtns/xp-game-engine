#version 450

layout(location=0) in vec3 in_color;
layout(location=0) out vec4 out_color;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

void main() {
    out_color = vec4(1.0, 0.0, 0.0, 1.0);
}
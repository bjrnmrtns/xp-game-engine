#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in vec3 in_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec3 out_position;
layout(location=1) out vec3 out_color;
layout(location=2) out vec3 out_normal;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 model;
    mat4 view;
    mat4 proj;
};

void main() {
    out_normal = mat3(transpose(inverse(view * model))) * in_normal;
    out_color = in_color;
    gl_Position = proj * view * model * vec4(in_position, 1.0);
    out_position = normalize(gl_Position).xyz;
}
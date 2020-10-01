#version 450

layout(location=0) in vec3 in_position;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

void main() {
    gl_Position = projection * view * vec4(in_position, 1.0);
}
#version 450

layout(location=0) in vec3 in_world_position;
layout(location=1) in vec3 in_world_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec4 out_color;

layout(std140, set=0, binding=0)
uniform Uniforms {
    mat4 view;
    mat4 proj;
};

layout(std140, set=0, binding=1)
buffer Transforms {
    mat4 models[];
};

void main()
{
    vec3 normal = normalize(in_world_normal);
    vec3 result = vec3(1.0, 1.0, 1.0);
    out_color = vec4(result, 1.0);
}
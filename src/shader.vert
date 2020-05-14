#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in uint in_color_id;

layout(location=0) out vec3 out_color;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 proj_view;
};

layout(set=1, binding=1)
buffer Instances {
    mat4 models[];
};

void main() {
    vec3 palette[3] = vec3[3](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));
    out_color = palette[in_color_id];
    gl_Position = proj_view * models[gl_InstanceIndex] * vec4(in_position, 1.0);
}
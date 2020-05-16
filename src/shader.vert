#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in vec3 in_normal;
layout(location=2) in uint in_color_id;

layout(location=0) out vec3 out_color;
layout(location=1) out vec3 out_normal;

layout(set=1, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

layout(set=1, binding=1)
buffer Instances {
    mat4 models[];
};

void main() {
    out_normal = mat3(transpose(inverse(view * models[gl_InstanceIndex]))) * in_normal;
//    out_normal = in_normal;
    vec3 palette[3] = vec3[3](vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0));
    out_color = palette[in_color_id];
    gl_Position = projection * view * models[gl_InstanceIndex] * vec4(in_position, 1.0);
}
#version 450

layout(location=0) in vec2 in_position;

layout(location=0) out vec3 out_color;

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
    out_color = vec3(1.0, 1.0, 0.0);
    //gl_Position = projection * view * models[gl_InstanceIndex] * vec4(in_position.x, 0.0, in_position.y, 1.0);
    gl_Position = projection * view * vec4(in_position, 0.0, 1.0).xzyw;
}
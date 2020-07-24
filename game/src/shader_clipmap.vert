#version 450

layout(location=0) in vec2 in_position;

layout(location=0) out vec3 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
};

layout(set=0, binding=1)
buffer Instances {
    mat4 models[];
};

layout(set = 0, binding = 2) uniform texture2D tex;
layout(set = 0, binding = 3) uniform sampler elevation_sampler;

void main() {
    out_color = vec3(1.0, 1.0, 0.0);
    float height = texture(sampler2D(tex, elevation_sampler), vec2(in_position.x / 16.0, in_position.y / 16.0)).r;
    gl_Position = projection * view * vec4(in_position, height, 1.0).xzyw;
}
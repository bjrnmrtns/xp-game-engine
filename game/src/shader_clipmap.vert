#version 450

layout(location=0) in vec2 in_position;

layout(location=0) out vec3 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
    vec3 camera_position;
};

layout(set=0, binding=1)
buffer Instances {
    mat4 models[];
};

layout(set = 0, binding = 2) uniform texture2D tex;
layout(set = 0, binding = 3) uniform sampler elevation_sampler;

void main() {
    vec2 snapped_camera_position = vec2(floor(camera_position.x), floor(camera_position.z));
    vec2 position = vec2(in_position.x + snapped_camera_position.x, in_position.y + snapped_camera_position.y);
    out_color = vec3(1.0, 1.0, 0.0);
    vec2 uv = vec2((position.x + 0.5) / 16.0, (position.y + 0.5) / 16.0);
    float height = texture(sampler2D(tex, elevation_sampler), uv).r;
    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
}
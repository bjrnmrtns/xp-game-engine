#version 450

layout(location=0) in vec2 in_position;

layout(location=0) out vec3 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
    vec3 camera_position;
};

struct Instance {
    uvec2 offset;
    uint clipmap_level;
    uint padding;
};

layout(set=0, binding=1)
buffer Instances {
    Instance instances[];
};

const vec3 COLOR_TABLE[8] = vec3[8](vec3(1.0, 1.0, 1.0f), vec3(1.0, 1.0, 0.0f), vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0));
const uint clipmap_size = 15;

layout(set = 0, binding = 2) uniform texture2D tex;
layout(set = 0, binding = 3) uniform sampler elevation_sampler;

void main() {
    vec2 instance_offset = instances[gl_InstanceIndex].offset;
    uint clipmap_level = instances[gl_InstanceIndex].clipmap_level;
    vec2 center = vec2(round(camera_position.x), round(camera_position.z));
    float clipmap_scale = pow(2, clipmap_level + 1);
    float clipmap_offset = (clipmap_size - 1) * clipmap_scale / 2;

    vec2 position = (in_position + instance_offset) * clipmap_scale - clipmap_offset + center;
    out_color = COLOR_TABLE[clipmap_level];
    vec2 uv = vec2(position.x / (clipmap_size * clipmap_scale), position.y / (clipmap_size * clipmap_scale));
    float height = texture(sampler2D(tex, elevation_sampler), uv).r;
    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
}
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
const float clipmap_size = 15.0;

layout(set = 0, binding = 2) uniform texture2D tex;
layout(set = 0, binding = 3) uniform sampler elevation_sampler;

void main() {
    uvec2 offset = instances[gl_InstanceIndex].offset;
    uint clipmap_level = instances[gl_InstanceIndex].clipmap_level;
    // -7 comes from (15 - 1) / 2, this still needs to be done in a proper way, by initalizing the vertices differently
    vec2 position = vec2(in_position.x + floor(camera_position.x) - 7, in_position.y + floor(camera_position.z) - 7) + offset;
    out_color = COLOR_TABLE[clipmap_level];
    vec2 uv = vec2(position.x / clipmap_size, position.y / clipmap_size); // tile_size = clipmap_size - 1, meaning 1 value is not used
    float height = texture(sampler2D(tex, elevation_sampler), uv).r;
    vec2 scaled_position = vec2(position.x * (clipmap_level + 1), position.y * (clipmap_level + 1));
    gl_Position = projection * view * vec4(scaled_position, height, 1.0).xzyw;
}
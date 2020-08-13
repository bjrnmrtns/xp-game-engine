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
    uvec2 offsets_level[];
};

const float clipmap_size = 15.0;

layout(set = 0, binding = 2) uniform texture2D tex;
layout(set = 0, binding = 3) uniform sampler elevation_sampler;

void main() {
    uvec2 offset = offsets_level[gl_InstanceIndex].xy;
    // -7 comes from (15 - 1) / 2, this still needs to be done in a proper way, by initalizing the vertices differently
    vec2 position = vec2(in_position.x + floor(camera_position.x) - 7, in_position.y + floor(camera_position.z) - 7) + offset;
    out_color = vec3(1.0, 1.0, 0.0);
    vec2 uv = vec2(position.x / clipmap_size, position.y / clipmap_size); // tile_size = clipmap_size - 1, meaning 1 value is not used
    float height = texture(sampler2D(tex, elevation_sampler), uv).r;
    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
}
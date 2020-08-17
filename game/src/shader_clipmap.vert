#version 450

layout(location=0) in ivec2 in_position;
layout(location=0) out vec3 out_color;

struct Instance {
    uvec2 top_left;
    uint clipmap_level;
    uint padding;
};

layout(set=0, binding=0)
uniform Uniforms {
    mat4 projection;
    mat4 view;
    vec3 camera_position;
};

layout(set=0, binding=1)
buffer Instances {
    Instance instances[];
};

layout(binding = 2, r32f) coherent uniform image2D heightmap;

const vec3 COLOR_TABLE[8] = vec3[8](vec3(1.0, 1.0, 1.0f), vec3(1.0, 1.0, 0.0f), vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0));

const uint clipmap_index_count = 255;
const float smallest_unit_size = 0.1;

float snap_grid_level(float val, float snap_size)
{
    return floor(val / snap_size) * snap_size;
}

void main() {
    uint clipmap_level = instances[gl_InstanceIndex].clipmap_level;
    float unit_size = smallest_unit_size * pow(2, clipmap_level + 1);
    vec2 top_left = instances[gl_InstanceIndex].top_left;
    vec2 center_snapped = vec2(snap_grid_level(camera_position.x, unit_size * 2), snap_grid_level(camera_position.z, unit_size * 2));
    float clipmap_correction = (clipmap_index_count - 3) * unit_size / 2;

    vec2 position = (in_position + top_left) * unit_size - clipmap_correction + center_snapped;
    out_color = COLOR_TABLE[clipmap_level];

    float u = mod(position.x / unit_size, clipmap_index_count);
    float v = mod(position.y / unit_size, clipmap_index_count);

    float height = imageLoad(heightmap, ivec2(u, v)).r;
    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
}
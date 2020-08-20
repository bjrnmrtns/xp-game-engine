#version 450

layout(location=0) in ivec2 index_offset_from_part;
layout(location=0) out vec3 out_color;

struct Instance {
    uvec2 part_offset_from_base;
    uint level;
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
    Instance clipmap_part_instances[];
};

layout(binding = 2, r32f) coherent uniform image3D heightmap;

const vec3 COLOR_TABLE[8] = vec3[8](vec3(1.0, 1.0, 1.0f), vec3(1.0, 1.0, 0.0f), vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0));

const uint clipmap_index_count = 15;
const float smallest_unit_size = 1.0;

float snap_grid_level(float val, float snap_size)
{
    return floor(val / snap_size) * snap_size;
}

float unit_size_for_level(float level)
{
    return pow(2, level) * smallest_unit_size;
}

vec2 snap_position_for_level(vec2 val, uint level)
{
    float snap_size = unit_size_for_level(level + 1);
    return vec2(floor(val.x / snap_size) * snap_size, floor(val.y / snap_size) * snap_size);
}

float base_offset(uint level) {
    return unit_size_for_level(level) * (clipmap_index_count - 3.0) / 2.0;
}

void main() {
    uint level = clipmap_part_instances[gl_InstanceIndex].level;
    float unit_size = unit_size_for_level(level);
    ivec2 part_offset_from_base = ivec2(clipmap_part_instances[gl_InstanceIndex].part_offset_from_base);

    vec2 snapped_center = snap_position_for_level(vec2(camera_position.x, camera_position.z), level);
    vec2 base_coordinate = snapped_center - vec2(base_offset(level), base_offset(level));

    vec2 position = base_coordinate + (part_offset_from_base + index_offset_from_part) * unit_size;
    ivec2 uv = part_offset_from_base + index_offset_from_part;
    float height = imageLoad(heightmap, ivec3(uv, level)).r;

    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
    out_color = COLOR_TABLE[level];
}
#version 450

layout(location=0) in ivec2 index_offset_from_part;
layout(location=0) out vec3 out_color;

struct Instance {
    uvec2 part_offset_from_base;
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

void main() {
    uint clipmap_level = clipmap_part_instances[gl_InstanceIndex].clipmap_level;
    float unit_size = smallest_unit_size * pow(2, clipmap_level + 1);
    ivec2 part_offset_from_base = ivec2(clipmap_part_instances[gl_InstanceIndex].part_offset_from_base);

    vec2 non_snapped_base_coordinate = vec2(camera_position.x - clipmap_index_count * unit_size / 2.0, camera_position.z - clipmap_index_count * unit_size / 2.0);
    vec2 base_coordinate = vec2(snap_grid_level(non_snapped_base_coordinate.x, unit_size * 2.0), snap_grid_level(non_snapped_base_coordinate.y, unit_size * 2.0));

    vec2 position = base_coordinate + (part_offset_from_base + index_offset_from_part) * unit_size;
    ivec2 uv = part_offset_from_base + index_offset_from_part;
    float height = imageLoad(heightmap, ivec3(uv, clipmap_level)).r;

    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
    out_color = COLOR_TABLE[clipmap_level];
}
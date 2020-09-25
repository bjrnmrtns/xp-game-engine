#version 450

layout(location=0) in ivec2 offset;
layout(location=0) out vec3 out_color;
layout(location=1) out vec3 out_normal;

struct Instance {
    uvec2 offset;
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
    Instance part[];
};

layout(binding = 2, rgba32f) coherent uniform image3D heightmap;

const vec3 COLOR_TABLE[8] = vec3[8](vec3(1.0, 1.0, 1.0f), vec3(1.0, 1.0, 0.0f), vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0));

const uint CM_N = 127;
const uint BASE_OFFSET = (CM_N - 3) / 2;
const float smallest_unit_size = 2.0;

float unit_size_for_level(uint level)
{
    return pow(2, float(level)) * smallest_unit_size;
}

int snap_to_index_for_level(float val, uint level) {
    float snap_size = unit_size_for_level(level + 1);
    return int(floor(val / snap_size) * 2.0);
}

void main() {
    uint level = part[gl_InstanceIndex].level;
    float unit_size = unit_size_for_level(level);
    ivec2 part_offset = ivec2(part[gl_InstanceIndex].offset);

    ivec2 center_index = ivec2(snap_to_index_for_level(camera_position.x, level), snap_to_index_for_level(camera_position.z, level));
    ivec2 pos_index = center_index - ivec2(BASE_OFFSET, BASE_OFFSET) + part_offset + offset;

    ivec2 uv = ivec2(uint(pos_index.x) % (CM_N + 1), uint(pos_index.y) % (CM_N + 1));
    float height = imageLoad(heightmap, ivec3(uv, level)).r;
    vec3 in_normal = imageLoad(heightmap, ivec3(uv, level)).gba;

    gl_Position = projection * view * vec4(vec2(pos_index) * unit_size, height, 1.0).xzyw;
    out_color = COLOR_TABLE[part[gl_InstanceIndex].padding];
    out_normal = mat3(transpose(inverse(view))) * in_normal;
}
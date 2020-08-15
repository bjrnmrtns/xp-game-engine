#version 450

layout(location=0) in vec2 in_position;
layout(location=0) out vec3 out_color;

struct Instance {
    uvec2 offset;
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

layout(set = 0, binding = 2) uniform texture2DRect tex;
layout(set = 0, binding = 3) uniform sampler2DRect elevation_sampler;

const vec3 COLOR_TABLE[8] = vec3[8](vec3(1.0, 1.0, 1.0f), vec3(1.0, 1.0, 0.0f), vec3(1.0, 0.0, 1.0), vec3(1.0, 0.0, 0.0), vec3(0.0, 1.0, 1.0), vec3(0.0, 1.0, 0.0), vec3(0.0, 0.0, 1.0), vec3(0.0, 0.0, 0.0));
const uint clipmap_size = 15;

float snap_grid_level(float val, float grid_scale)
{
    return floor(val / grid_scale) * grid_scale;
}

void main() {
    vec2 instance_offset = instances[gl_InstanceIndex].offset;
    uint clipmap_level = instances[gl_InstanceIndex].clipmap_level;
    float clipmap_scale = pow(2, clipmap_level + 1);
    vec2 center_snapped = vec2(snap_grid_level(camera_position.x, clipmap_scale * 2), snap_grid_level(camera_position.z, clipmap_scale * 2));
    float clipmap_offset = (clipmap_size - 3) * clipmap_scale / 2;

    vec2 position = (in_position + instance_offset) * clipmap_scale - clipmap_offset + center_snapped;
    out_color = COLOR_TABLE[clipmap_level];
    vec2 uv = vec2(position.x / (clipmap_size * clipmap_scale), position.y / (clipmap_size * clipmap_scale));
//    float height = texture(sampler2D(tex, elevation_sampler), uv).r;
    float height = 0.0;//texelFetch(elevation_sampler, ivec2(0,0)).r;
    gl_Position = projection * view * vec4(position, height, 1.0).xzyw;
}
#version 450

layout(location=0) in vec3 in_world_position;
layout(location=1) in vec3 in_world_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec4 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 model;
    mat4 view;
    mat4 proj;
    vec4 world_light_position;
    vec4 light_color;
};

/*void main() {
    vec3 view_light = mat3(view) * light;
    float lum = max(dot(normalize(in_normal), normalize(view_light)), 0.0);
    out_color = vec4(in_color * (0.2 + 0.8 * lum), 1.0);
}
*/
void main()
{
    // ambient calculation
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * vec3(light_color);

    // diffuse calculation
    vec3 world_normal = normalize(in_world_normal);
    vec3 world_light_direction = normalize(vec3(world_light_position) - in_world_position);
    float diff = max(dot(world_normal, world_light_direction), 0.0);
    vec3 diffuse = diff * vec3(light_color);

    vec3 result = (diffuse) * in_color;
    out_color = vec4(result, 1.0);
}
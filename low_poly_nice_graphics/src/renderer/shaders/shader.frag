#version 450

layout(location=0) in vec3 in_position;
layout(location=1) in vec3 in_color;
layout(location=2) in vec3 in_normal;

layout(location=0) out vec4 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 model;
    mat4 view;
    mat4 proj;
};

const vec3 light = vec3(1.0, 1.0, -1.0);
const vec3 light_color = vec3(1.0, 1.0, 1.0);

/*void main() {
    vec3 view_light = mat3(view) * light;
    float lum = max(dot(normalize(in_normal), normalize(view_light)), 0.0);
    out_color = vec4(in_color * (0.2 + 0.8 * lum), 1.0);
}
*/
void main()
{
    // diffuse calculation
    vec3 light_position = mat3(view) * light;
    vec3 light_direction = normalize(light_position - in_position);
    vec3 normal = normalize(in_normal);
    float diff = max(dot(normal, light_direction), 0.0);
    vec3 diffuse = diff * light_color;

    // ambient calculation
    float ambient_strength = 0.1;
    vec3 ambient = ambient_strength * light_color;

    vec3 result = (ambient + diffuse) * in_color;
    out_color = vec4(result, 1.0);
}
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
    vec4 world_camera_position;

    vec4 world_light_position;
    vec4 light_ambient;
    vec4 light_diffuse;
    vec4 light_specular;

    vec4 directional_direction;
    vec4 directional_ambient;
    vec4 directional_diffuse;
    vec4 directional_specular;

    vec4 point_position;
    vec4 point_ambient;
    vec4 point_diffuse;
    vec4 point_specular;
    vec4 constant_linear_specular; // first three components (xyz) represent constant linear and quadratic

    vec4 spot_position;
    vec4 spot_direction;
    vec4 cut_off; // first component (x) is cut_off

    vec4 material_ambient;
    vec4 material_diffuse;
    vec4 material_specular;
    float material_shininess;
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
    vec3 ambient = vec3(light_ambient) * vec3(material_ambient);

    // diffuse calculation
    vec3 world_normal = normalize(in_world_normal);
    vec3 world_light_direction = normalize(vec3(world_light_position) - in_world_position);
    float diff = max(dot(world_normal, world_light_direction), 0.0);
    vec3 diffuse = vec3(light_diffuse) * diff * vec3(material_diffuse);

    // specular calculation
    vec3 view_direction = normalize(vec3(world_camera_position) - in_world_position);
    vec3 reflect_direction = reflect(-world_light_direction, world_normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material_shininess);
    vec3 specular = vec3(light_specular) * spec * vec3(material_specular);

    vec3 result = (ambient + diffuse + specular) * in_color;
    out_color = vec4(result, 1.0);
}
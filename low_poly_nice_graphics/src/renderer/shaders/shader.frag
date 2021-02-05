#version 450

layout(location=0) in vec3 in_world_position;
layout(location=1) in vec3 in_world_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec4 out_color;

// Padding (named p<n> bytes placed for own clarity because of alignment contraints see: https://learnopengl.com/Advanced-OpenGL/Advanced-GLSL
// alignment is implicit
struct DirectionalLight
{
    vec3 direction; float p0;
    vec3 ambient; float p1;
    vec3 diffuse; float p2;
    vec3 specular;
};
struct SpotLight
{
    vec3 position; float p0;
    vec3 direction; float p1;
    vec3 ambient; float p2;
    vec3 diffuse; float p3;
    vec3 specular;
    float constant;
    float linear;
    float quadratic;
    float cut_off_inner;
    float cut_off_outer;
};

struct PointLight
{
    vec3 position; float p0;
    vec3 ambient; float p1;
    vec3 diffuse; float p2;
    vec3 specular;
    float constant;
    float linear;
    float quadratic;
};

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
    vec4 constant_linear_quadratic; // first three components (xyz) represent constant linear and quadratic

    vec4 spot_position;
    vec4 spot_direction;
    vec4 cut_off; // first component (x) is cut_off

    vec4 material_ambient;
    vec4 material_diffuse;
    vec4 material_specular;
    float material_shininess;
};
const uint MAX_NR_OF_DIRECTIONAL_LIGHTS = 1;
layout(set=1, binding=0)
uniform DirectionalLightBlock {
    DirectionalLight directional_lights[MAX_NR_OF_DIRECTIONAL_LIGHTS];
};

const uint MAX_NR_OF_SPOT_LIGHTS = 10;
layout(set=2, binding=0)
uniform SpotLightBlock {
    SpotLight spot_lights[MAX_NR_OF_SPOT_LIGHTS];
};

const uint MAX_NR_OF_POINT_LIGHTS = 10;
layout(set=3, binding=0)
uniform PointLightBlock {
    PointLight point_lights[MAX_NR_OF_POINT_LIGHTS];
};

vec3 calculate_directional_light(DirectionalLight light, vec3 normal, vec3 view_direction)
{
    // negate light direction -> we want direction towards light
    vec3 light_direction = normalize(-light.direction);
    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material_shininess);

    vec3 ambient  = light.ambient * vec3(material_ambient);
    vec3 diffuse = light.diffuse * diff * vec3(material_diffuse);
    vec3 specular = light.specular * spec * vec3(material_specular);

    return ambient + diffuse + specular;
}

vec3 calculate_spot_light(SpotLight light, vec3 normal, vec3 frag_position, vec3 view_direction)
{
    vec3 light_direction = normalize(light.position - frag_position);
    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material_shininess);
    // attenuation
    float distance = length(light.position - frag_position);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    // spotlight intensity
    float theta = dot(light_direction, normalize(-light.direction));
    float epsilon = light.cut_off_inner - light.cut_off_outer;
    float intensity = clamp ((theta - light.cut_off_outer) / epsilon, 0.0, 1.0);
    vec3 ambient = light.ambient * vec3(material_ambient) * attenuation * intensity;
    vec3 diffuse = light.diffuse * diff * vec3(material_diffuse) * attenuation * intensity;
    vec3 specular = light.specular * spec * vec3(material_specular) * attenuation * intensity;
    return ambient + diffuse + specular;
}

vec3 calculate_point_light(PointLight light, vec3 normal, vec3 frag_position, vec3 view_direction)
{
    vec3 light_direction = normalize(light.position - frag_position);
    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 reflect_direction = reflect(-light_direction, normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material_shininess);
    // attenuation
    float distance = length(light.position - frag_position);
    float attenuation = 1.0 / (light.constant + light.linear * distance + light.quadratic * (distance * distance));
    vec3 ambient = light.ambient * vec3(material_ambient) * attenuation;
    vec3 diffuse = light.diffuse * diff * vec3(material_diffuse) * attenuation;
    vec3 specular = light.specular * spec * vec3(material_specular) * attenuation;
    return ambient + diffuse + specular;
}

void main()
{
    // ambient calculation
    vec3 ambient = vec3(light_ambient) * vec3(material_ambient);

    // diffuse calculation
    vec3 world_normal = normalize(in_world_normal);
    vec3 light_direction = normalize(vec3(spot_position) - in_world_position);//normalize(-vec3(directional_direction));
    float diff = max(dot(world_normal, light_direction), 0.0);
    vec3 diffuse = vec3(light_diffuse) * diff * vec3(material_diffuse);

    // point-light attenuation
    float distance = length(vec3(point_position) - in_world_position);
    float attenuation = 1.0 / (constant_linear_quadratic[0] + constant_linear_quadratic[1] * distance + constant_linear_quadratic[2] * (distance * distance));

    // spot light
    float theta = dot(light_direction, normalize(-vec3(spot_direction)));
    float epsilon = cut_off[0] - cut_off[1];
    float intensity = clamp((theta, cut_off[1]) / epsilon, 0.0, 1.0);

    // specular calculation
    vec3 view_direction = normalize(vec3(world_camera_position) - in_world_position);
    vec3 reflect_direction = reflect(-light_direction, world_normal);
    float spec = pow(max(dot(view_direction, reflect_direction), 0.0), material_shininess);
    vec3 specular = vec3(light_specular) * spec * vec3(material_specular);

    ambient *= attenuation;
    diffuse *= attenuation * intensity;
    specular *= attenuation * intensity;

    vec3 result = (ambient + diffuse + specular) * in_color;
    out_color = vec4(result, 1.0);
}
#version 450

layout(location=0) in vec3 in_world_position;
layout(location=1) in vec3 in_world_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec4 out_color;

// Alignment rules see: https://learnopengl.com/Advanced-OpenGL/Advanced-GLSL
struct DirectionalLight
{
    vec4 direction;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
};

struct SpotLight
{
    vec4 position;
    vec4 direction;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float cons;
    float linear;
    float quadratic;
    float cut_off_inner;
    float cut_off_outer;
    float p0, p1, p2;
};

struct PointLight
{
    vec4 position;
    vec4 ambient;
    vec4 diffuse;
    vec4 specular;
    float cons;
    float linear;
    float quadratic;
    float p0;
};

layout(std140, set=0, binding=0)
uniform Uniform {
    mat4 view;
    mat4 proj;
    vec4 world_camera_position;
    vec4 material_specular;
    float material_shininess;
    uint nr_of_directional_lights;
    uint nr_of_spot_lights;
    uint nr_of_point_lights;
};
const uint MAX_NR_OF_DIRECTIONAL_LIGHTS = 1;
layout(std140, set=0, binding=1)
uniform DirectionalLightBlock {
    DirectionalLight directional_lights[MAX_NR_OF_DIRECTIONAL_LIGHTS];
};

const uint MAX_NR_OF_SPOT_LIGHTS = 10;
layout(std140, set=0, binding=2)
uniform SpotLightBlock {
    SpotLight spot_lights[MAX_NR_OF_SPOT_LIGHTS];
};

const uint MAX_NR_OF_POINT_LIGHTS = 10;
layout(std140, set=0, binding=3)
uniform PointLightBlock {
    PointLight point_lights[MAX_NR_OF_POINT_LIGHTS];
};

layout(std140, set=0, binding=4)
buffer Instance {
    mat4 models[];
};

vec3 calculate_directional_light(uint i, vec3 normal, vec3 view_direction)
{
    // negate light direction -> we want direction towards light
    vec3 light_direction = normalize(-directional_lights[i].direction.xyz);
    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 halfway_direction = normalize(light_direction + view_direction);
    float spec = pow(max(dot(view_direction, halfway_direction), 0.0), material_shininess);

    vec3 ambient  = directional_lights[i].ambient.xyz * in_color;
    vec3 diffuse = directional_lights[i].diffuse.xyz * diff * in_color;
    vec3 specular = directional_lights[i].specular.xyz * spec * material_specular.xyz;

    return ambient + diffuse + specular;
}

vec3 calculate_spot_light(uint i, vec3 normal, vec3 frag_position, vec3 view_direction)
{
    vec3 light_direction = normalize(spot_lights[i].position.xyz - frag_position);

    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 halfway_direction = normalize(light_direction + view_direction);
    float spec = pow(max(dot(view_direction, halfway_direction), 0.0), material_shininess);
    // attenuation
    float distance = distance(spot_lights[i].position.xyz, frag_position);
    float attenuation = 1.0 / (spot_lights[i].cons + spot_lights[i].linear * distance + spot_lights[i].quadratic * (distance * distance));
    // spotlight intensity
    float theta = dot(light_direction, normalize(-spot_lights[i].direction.xyz));
    float epsilon = spot_lights[i].cut_off_inner - spot_lights[i].cut_off_outer;
    float intensity = clamp ((theta -spot_lights[i].cut_off_outer) / epsilon, 0.0, 1.0);
    vec3 ambient = spot_lights[i].ambient.xyz * in_color * attenuation * intensity;
    vec3 diffuse = spot_lights[i].diffuse.xyz * diff * in_color * attenuation * intensity;
    vec3 specular = spot_lights[i].specular.xyz * spec * material_specular.xyz * attenuation * intensity;
    return ambient + diffuse + specular;
}

vec3 calculate_point_light(uint i, vec3 normal, vec3 frag_position, vec3 view_direction)
{
    vec3 light_direction = normalize(point_lights[i].position.xyz - frag_position);
    // diffuse
    float diff = max(dot(normal, light_direction), 0.0);
    // specular
    vec3 halfway_direction = normalize(light_direction + view_direction);
    float spec = pow(max(dot(view_direction, halfway_direction), 0.0), material_shininess);
    // attenuation
    float distance = distance(point_lights[i].position.xyz, frag_position);
    float attenuation = 1.0 / (point_lights[i].cons + point_lights[i].linear * distance + point_lights[i].quadratic * (distance * distance));

    vec3 ambient = point_lights[i].ambient.xyz * in_color * attenuation;
    vec3 diffuse = point_lights[i].diffuse.xyz * diff * in_color * attenuation;
    vec3 specular = point_lights[i].specular.xyz * spec * material_specular.xyz * attenuation;
    return ambient + diffuse + specular;
}

void main()
{
    vec3 normal = normalize(in_world_normal);
    vec3 view_direction = normalize(world_camera_position.xyz - in_world_position);

    vec3 result = vec3(0.0, 0.0, 0.0);

    for(uint i = 0; i < nr_of_directional_lights; i++) {
        result += calculate_directional_light(i, normal, view_direction);
    }
    for(uint i = 0; i < nr_of_spot_lights; i++) {
        result += calculate_spot_light(i, normal, in_world_position, view_direction);
    }
    for(uint i = 0; i < nr_of_point_lights; i++) {
        result += calculate_point_light(i, normal, in_world_position, view_direction);
    }
    // gamma correction
    float gamma = 2.2;
    out_color = vec4(pow(result, vec3(1.0 / gamma)), 1.0);
}
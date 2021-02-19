#version 450

layout(location=0) in vec3 in_model_position;
layout(location=1) in vec3 in_model_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec3 out_world_position;
layout(location=1) out vec3 out_world_normal;
layout(location=2) out vec3 out_color;

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
uniform Uniforms {
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

struct Transform {
    mat4 model;
    vec4 material_specular;
    float material_shininess;
};

layout(std140, set=0, binding=4)
buffer Transforms {
    mat4 models[];
};

void main() {
    out_world_position = vec3(models[gl_InstanceIndex] * vec4(in_model_position, 1.0));
    // TODO: doing inverse for every vertex is expensive, this can be done once per mesh on the cpu
    out_world_normal = mat3(transpose(inverse(models[gl_InstanceIndex]))) * in_model_normal; // now normal is world coordinates, as a normal vector is only a direction we remove the translation part of the model matrix (mat4 -> mat3, does that)
    //out_world_normal = in_model_normal; // not entirely correct, but because we are using identity matrix for model still, it is fine
    out_color = in_color;
    gl_Position = proj * view * models[gl_InstanceIndex] * vec4(in_model_position, 1.0);
}


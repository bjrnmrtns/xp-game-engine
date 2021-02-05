#version 450

layout(location=0) in vec3 in_model_position;
layout(location=1) in vec3 in_model_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec3 out_world_position;
layout(location=1) out vec3 out_world_normal;
layout(location=2) out vec3 out_color;

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
    vec3 world_camera_position; float p0;

    vec3 material_ambient; float p1;
    vec3 material_diffuse; float p2;
    vec3 material_specular;
    float material_shininess;
};
const uint MAX_NR_OF_DIRECTIONAL_LIGHTS = 1;
layout(set=0, binding=1)
uniform DirectionalLightBlock {
    DirectionalLight directional_lights[MAX_NR_OF_DIRECTIONAL_LIGHTS];
};

const uint MAX_NR_OF_SPOT_LIGHTS = 10;
layout(set=0, binding=2)
uniform SpotLightBlock {
    SpotLight spot_lights[MAX_NR_OF_SPOT_LIGHTS];
};

const uint MAX_NR_OF_POINT_LIGHTS = 10;
layout(set=0, binding=3)
uniform PointLightBlock {
    PointLight point_lights[MAX_NR_OF_POINT_LIGHTS];
};

void main() {
    out_world_position = vec3(model * vec4(in_model_position, 1.0));
    // TODO: doing inverse for every vertex is expensive, this can be done once per mesh on the cpu
    out_world_normal = mat3(transpose(inverse(model))) * in_model_normal; // now normal is world coordinates, as a normal vector is only a direction we remove the translation part of the model matrix (mat4 -> mat3, does that)
    //out_world_normal = in_model_normal; // not entirely correct, but because we are using identity matrix for model still, it is fine
    out_color = in_color;
    gl_Position = proj * view * model * vec4(in_model_position, 1.0);
}


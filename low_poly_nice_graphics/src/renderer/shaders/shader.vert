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
    vec3 directional; float p0;
    vec3 ambient; float p1;
    vec3 diffuse; float p2;
    vec3 specular; float p3;
};
const uint MAX_NR_OF_DIRECTIONAL_LIGHTS = 1;
struct SpotLight
{
    vec3 position; float p0;
    vec3 direction;
    float cut_off_inner;
    float cut_off_outer;
};
const uint MAX_NR_OF_SPOT_LIGHTS = 10;

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
const uint MAX_NR_OF_POINT_LIGHTS = 10;

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
    DirectionalLight directional_lights[MAX_NR_OF_DIRECTIONAL_LIGHTS];
    SpotLight spot_lights[MAX_NR_OF_SPOT_LIGHTS];
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


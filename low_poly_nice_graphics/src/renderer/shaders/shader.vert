#version 450

layout(location=0) in vec3 in_model_position;
layout(location=1) in vec3 in_model_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec3 out_world_position;
layout(location=1) out vec3 out_world_normal;
layout(location=2) out vec3 out_color;

layout(set=0, binding=0)
uniform Uniforms {
    mat4 model;
    mat4 view;
    mat4 proj;
    vec4 world_light_position;
    vec4 light_color;
};

/*void main() {
    out_normal = mat3(transpose(inverse(view * model))) * in_normal;
    out_color = in_color;
    gl_Position = proj * view * model * vec4(in_position, 1.0);
    out_position = vec3(view * model * vec4(in_position, 1.0));
}
*/
void main() {
    out_world_position = vec3(model * vec4(in_model_position, 1.0));
    out_world_normal =  in_model_normal; // not entirely correct, but because we are using identity matrix for model still, it is fine
    out_color = in_color;
    gl_Position = proj * view * model * vec4(in_model_position, 1.0);
}


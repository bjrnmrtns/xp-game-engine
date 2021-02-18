#version 450

layout(location=0) in vec3 in_model_position;
layout(location=1) in vec3 in_model_normal;
layout(location=2) in vec3 in_color;

layout(location=0) out vec3 out_world_position;
layout(location=1) out vec3 out_world_normal;
layout(location=2) out vec3 out_color;

layout(std140, set=0, binding=0)
uniform Uniforms {
    mat4 view;
    mat4 proj;
};

layout(std140, set=0, binding=1)
buffer Transforms {
    mat4 models[];
};

void main() {
    out_world_position = vec3(models[gl_InstanceIndex] * vec4(in_model_position, 1.0));
    // TODO: doing inverse for every vertex is expensive, this can be done once per mesh on the cpu
    out_world_normal = mat3(transpose(inverse(models[gl_InstanceIndex]))) * in_model_normal; // now normal is world coordinates, as a normal vector is only a direction we remove the translation part of the model matrix (mat4 -> mat3, does that)
    out_color = in_color;
    gl_Position = proj * view * models[gl_InstanceIndex] * vec4(in_model_position, 1.0);
}


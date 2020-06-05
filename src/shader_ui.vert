#version 450

layout(set = 0, binding = 0) uniform Uniforms {
  mat4 projection;
};

layout(location = 0) in vec2 in_pos;
layout(location = 1) in vec2 in_uv;
layout(location = 2) in uint in_color;

layout(location = 0) out vec2 out_uv;
layout(location = 1) out vec4 out_color;

// Built-in:
// vec4 gl_Position

void main() {
  out_uv = in_uv;
  out_color = vec4(in_color & 0xFF, (in_color >> 8) & 0xFF, (in_color >> 16) & 0xFF, (in_color >> 24) & 0xFF) / 255.0;
  gl_Position = projection * vec4(in_pos.xy, 0.0, 1.0);
}
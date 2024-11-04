#version 330 core
in vec3 pos_attr;
in vec2 tex_attr;

out vec2 tex_coords;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * vec4(pos_attr, 1.0);
  tex_coords = tex_attr;
}

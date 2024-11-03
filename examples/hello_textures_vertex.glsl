#version 330 core
in vec3 apos;
in vec3 acol;
in vec2 atex;
out vec3 frag_color;
out vec2 tex_coord;

uniform mat4 transform;

void main() {
  gl_Position = transform * vec4(apos, 1.0f);
  tex_coord = atex;
  frag_color = acol;
}

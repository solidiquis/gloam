#version 330 core
in vec3 apos;
in vec3 acol;
in vec2 atex;
out vec3 frag_color;
out vec2 tex_coord;

void main() {
  gl_Position = vec4(apos, 1.0f);
  tex_coord = atex;
  frag_color = acol;
}

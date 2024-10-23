#version 330 core
in vec3 frag_color;
in vec2 tex_coord;
out vec4 FragColor;

uniform sampler2D texture1;

void main() {
  FragColor = texture(texture1, tex_coord);
}

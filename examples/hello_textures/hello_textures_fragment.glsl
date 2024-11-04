#version 330 core
in vec3 frag_color;
in vec2 tex_coord;
out vec4 FragColor;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main() {
  FragColor = mix(texture(texture1, tex_coord), texture(texture2, tex_coord), 0.2f) * vec4(frag_color, 1.0f);
}

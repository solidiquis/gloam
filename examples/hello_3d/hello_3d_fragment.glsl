#version 330 core
in vec2 tex_coords;
out vec4 FragColor; 

uniform sampler2D metal;
uniform sampler2D sift;

void main() {
  FragColor = mix(texture(metal, tex_coords), texture(sift, tex_coords), 0.6f);
}

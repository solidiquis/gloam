#version 330 core

out vec4 outputColor;

uniform float red;
uniform float green;
uniform float blue;

void main() {
  outputColor = vec4(red, green, blue, 1.0);
}

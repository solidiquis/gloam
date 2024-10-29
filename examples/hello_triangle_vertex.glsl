#version 330 core
in vec3 aPosition;
in vec3 aColor;

out vec3 fragmentColor;

void main() {
  gl_Position = vec4(aPosition, 1.0f);
  fragmentColor = aColor;
}

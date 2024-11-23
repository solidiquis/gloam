#version 330 core
in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 fragColor;
out vec3 fragPosition;
out vec3 fragNormal;

uniform mat3 normalMatrix;
uniform mat4 modelMatrix;
uniform mat4 viewMatrix;
uniform mat4 projectionMatrix;

void main() {
  gl_Position = projectionMatrix * viewMatrix * modelMatrix * vec4(position, 1.0);
  fragPosition = vec3(modelMatrix * vec4(position, 1.0));
  fragNormal = normalize(normalMatrix * normal);
  fragColor = color;
}

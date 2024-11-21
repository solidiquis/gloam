#version 330 core
in vec3 position;
in vec3 color;
in vec3 normal;

out vec3 fragColor;
out vec3 fragNormal;
out vec3 fragPosition;

uniform mat3 normalMatrix;
uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

void main() {
  gl_Position = projection * view * model * vec4(position, 1.0);
  fragColor = color;
  fragPosition = vec3(model * vec4(position, 1.0));
  fragNormal = normalize(normalMatrix * normal);
}

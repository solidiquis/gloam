#version 330 core

in vec3 fragColor;
in vec3 fragPosition;
in vec3 fragNormal;

out vec4 finalColor;

uniform float ambientLightIntensity;
uniform float specularLightIntensity;
uniform vec3 cameraPosition;
uniform vec3 lightPosition;
uniform vec3 lightColor;

void main() {
  vec3 ambient = ambientLightIntensity * lightColor;

  vec3 lightDirection = normalize(lightPosition - fragPosition);
  float diffuseStrength = max(dot(lightDirection, fragNormal), 0.0);
  vec3 diffuse = diffuseStrength * lightColor;

  vec3 viewDirection = normalize(cameraPosition - fragPosition);
  vec3 reflectDirection = reflect(-lightDirection, fragNormal);
  float specularStrength = pow(max(dot(viewDirection, reflectDirection), 0.0), 32);
  vec3 specular = specularLightIntensity * specularStrength * lightColor;  

  finalColor = vec4((ambient + diffuse + specular) * fragColor, 1.0);
}

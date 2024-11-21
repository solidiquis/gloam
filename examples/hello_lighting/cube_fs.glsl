#version 330 core
in vec3 fragColor; 
in vec3 fragNormal;
in vec3 fragPosition;

out vec4 FragColor;

uniform vec3 cameraPosition;
uniform vec3 lightPosition;
uniform vec3 lightColor;
uniform float ambientLightIntensity;
uniform float specularLightIntensity;

void main() {
  // ambient
  vec3 ambientLight = ambientLightIntensity * lightColor;

  // diffuse
  vec3 lightDirection = normalize(lightPosition - fragPosition);
  float diffuseStrength = max(dot(fragNormal, lightDirection), 0.0);
  vec3 diffuse = diffuseStrength * lightColor;

  // specular
  vec3 viewDirection = normalize(cameraPosition - fragPosition);
  vec3 reflectDirection = reflect(-lightDirection, fragNormal); 
  float specularStrength = pow(max(dot(viewDirection, reflectDirection), 0.0), 32);
  vec3 specular = specularLightIntensity * specularStrength * lightColor;  

  FragColor = vec4((ambientLight + diffuse + specular) * fragColor, 1.0);
}

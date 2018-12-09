#version 330 core
out vec4 FragColor;

uniform vec3 lightColor;
uniform float ambientStrength;
uniform float specularStrength;
uniform vec3 lightPos;
uniform vec3 viewPos;

in vec3 FragPos;
in vec3 Normal;
in vec2 TexCoords;

uniform sampler2D diffuseTexture;

void main()
{
  vec3 ambient = ambientStrength * vec3(texture(diffuseTexture, TexCoords));

  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - FragPos);

  vec3 viewDir = normalize(viewPos - FragPos);
  vec3 reflectDir = reflect(-lightDir, norm);
  float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32);
  vec3 specular = specularStrength * spec * lightColor;

  float dotProduct = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = dotProduct * lightColor * vec3(texture(diffuseTexture, TexCoords));

  vec3 result = (ambient + diffuse + specular);
	FragColor = vec4(result, 1.0);
}
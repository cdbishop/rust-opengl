#version 330 core
out vec4 FragColor;

uniform vec3 objectColor;
uniform vec3 lightColor;
uniform float ambientStrength;
uniform vec3 lightPos;

in vec3 FragPos;
in vec3 Normal;

void main()
{
  vec3 ambient = ambientStrength * lightColor;

  vec3 norm = normalize(Normal);
  vec3 lightDir = normalize(lightPos - FragPos);


  float dotProduct = max(dot(norm, lightDir), 0.0);
  vec3 diffuse = dotProduct * lightColor;

  vec3 result = (ambient + diffuse) * objectColor;
	FragColor = vec4(result, 1.0);
}
#version 330 core
layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNormal;
layout (location = 2) in vec2 aTexCoords;

uniform mat4 transform;
uniform mat4 model;

out vec3 FragPos;
out vec3 Normal;
out vec2 TexCoords;

void main()
{
  FragPos = vec3(model * vec4(aPos, 1.0));

  gl_Position = transform * vec4(aPos, 1.0);
  Normal = mat3(transpose(inverse(model))) * aNormal;
  TexCoords = aTexCoords;
}
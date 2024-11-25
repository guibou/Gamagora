#version 460

// 0 c'est la reference pour opengl de cet attribut de vertex
layout(location=0) in vec3 position;

layout(location=0) uniform mat4 transformation;

void main()
{
   // Position de votre point entre [-1; 1] dans l'espace de l'écran
   // le 1 c'est la coordonnée homogene
   gl_Position = vec4(position, 1) * transformation;
}

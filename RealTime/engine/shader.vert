#version 460

// 0 c'est la reference pour opengl de cet attribut de vertex
layout(location=0) in vec2 position;
layout(location=1) in vec3 color;

out vec4 color_frag;

void main()
{
   // Position de votre point entre [-1; 1] dans l'espace de l'écran
   // le 1 c'est la coordonnée homogene
   gl_Position = vec4(position, 0, 1);
   gl_PointSize = 20;

   color_frag = vec4(color, 1);
}

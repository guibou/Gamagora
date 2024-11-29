#version 460

// 0 c'est la reference pour opengl de cet attribut de vertex
// Dans quel espace est "position?"
// Dans l'espace de definition du mesh
layout(location=0) in vec3 position;
layout(location=1) in vec3 normal;
layout(location=2) in vec2 uv;

layout(location=0) uniform mat4 transformation;
layout(location=3) uniform mat4 object_to_world;
layout(location=4) uniform mat4 object_to_world_normal;

out vec4 position_in_world_space;
out vec3 normal_in_world_space;
out vec2 uv_frag;

void main()
{
   // Position de votre point entre [-1; 1] dans l'espace de l'écran
   // le 1 c'est la coordonnée homogene
   gl_Position = vec4(position, 1) * transformation;
   position_in_world_space = vec4(position, 1) * object_to_world;

   // Be careful, normal transformation!
   // We cannot transform with the original matrice, because otherwise normals are deformed!
   normal_in_world_space = (vec4(normal, 0) * object_to_world_normal).xyz;

   uv_frag = uv;
}

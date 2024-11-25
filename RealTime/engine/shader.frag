#version 460


// Doit sortir un out vec4 qui represente la couleur
out vec4 color;

layout(location=1) uniform vec4 color_uniform;

void main()
{
    // Le 1 c'est l'alpha!
    color = color_uniform;
}

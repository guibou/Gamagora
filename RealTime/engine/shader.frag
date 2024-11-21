#version 460

// Entrée ici est une interpolation de la sortie du vertex shader
in vec4 color_frag;

// Doit sortir un out vec4 qui represente la couleur
out vec4 color;

void main()
{
    // Le 1 c'est l'alpha!
    color = color_frag;
}

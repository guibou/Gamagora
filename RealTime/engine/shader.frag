#version 460

// Doit sortir un out vec4 qui represente la couleur
out vec4 color;

void main()
{
    // Le 1 c'est l'alpha!
    color = vec4(0, 0, 1, 1);
}

#include <glad/glad.h>
#include <GLFW/glfw3.h>
#include <glm/vec3.hpp>
#define GLM_ENABLE_EXPERIMENTAL
#include <glm/gtx/transform.hpp>

#include <vector>
#include <iostream>
#include <random>
#include <sstream>
#include <fstream>
#include <string>
#include <algorithm>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#include "shader.h"

static void error_callback(int /*error*/, const char* description)
{
    std::cerr << "Error: " << description << std::endl;
}

static void key_callback(GLFWwindow* window, int key, int /*scancode*/, int action, int /*mods*/)
{
    if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
        glfwSetWindowShouldClose(window, GLFW_TRUE);
}

void APIENTRY opengl_error_callback(GLenum source,
        GLenum type,
        GLuint id,
        GLenum severity,
        GLsizei length,
        const GLchar *message,
        const void *userParam)
{
    std::cout << message << std::endl;
}

int main(void)
{
    // Creation du context openGL avec glfw
    GLFWwindow* window;
    glfwSetErrorCallback(error_callback);

    if (!glfwInit())
        exit(EXIT_FAILURE);

    glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
    glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
    glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, GLFW_TRUE);

    window = glfwCreateWindow(800, 800, "Gamagora triangle", NULL, NULL);

    if (!window)
    {
        glfwTerminate();
        exit(EXIT_FAILURE);
    }

    glfwSetKeyCallback(window, key_callback);
    glfwMakeContextCurrent(window);
    glfwSwapInterval(1);

    // Context OpenGL ouvert!

    // Ici on demande au driver d'initialiser les fonctions OpenGL
    // J'ai utilisé glad: https://glad.dav1d.de/
    // Alternatives: glew
    if(!gladLoadGL()) {
        std::cerr << "Something went wrong!" << std::endl;
        exit(-1);
    }

    std::cout << glGetString(GL_VERSION) << std::endl;

    // Callbacks
    // THIS IS IMPORTANT
    glDebugMessageCallback(opengl_error_callback, nullptr);
    // Cela ralenti l'execution MAIS cela rend les appels à
    // opengl_error_callback synchro avec l'appel de fonction
    glEnable(GL_DEBUG_OUTPUT_SYNCHRONOUS);

    // Shader
    const auto vertex = MakeShader(GL_VERTEX_SHADER, "shader.vert");
    const auto fragment = MakeShader(GL_FRAGMENT_SHADER, "shader.frag");
    const auto program = AttachAndLink({vertex, fragment});

    glUseProgram(program);

    // Buffers
    GLuint vbo, vao;

    // Attention à l'api avec des pointeurs
    // Souvent, en python par example:
    // [vbo] = glCreateBuffers(1)
    //
    // Nomenclature OpenGL:
    // gl[Create, Gen]XXXX
    // example:
    //    glCreateBuffers: nouvelle api, sans binds, avec des "Named" functions.
    //    glGenBuffers (pre Direct State Access "ancienne API", associée avec des "binds")
    glCreateBuffers(1, &vbo);
    float vertices[] = {-0.5, -0.5, 0.5, 0.5, 1, 0};
    // Copy des données dans le GPU
    glNamedBufferData(vbo, sizeof(vertices), vertices, GL_DYNAMIC_DRAW);
    std::cout << "Nb points: " << sizeof(vertices) / sizeof(float) / 2 << std::endl;

    // Bindings
    glCreateVertexArrays(1, &vao);
    const auto index = 0;
    const auto binding_point = 0;

    // Position
    // An active l'index (arbitraire, 0, c'est un choix) dans le vao
    glEnableVertexArrayAttrib(vao, index);
    // Associé à cet index, on aura des floats, par packet de 2
    glVertexArrayAttribFormat(vao, index, 2, GL_FLOAT, GL_FALSE, 0);
    // On va associer à l'index, le point "location" dans le shader
    glVertexArrayAttribBinding(vao, index, binding_point);

    glVertexArrayVertexBuffer(vao, binding_point, vbo, 0, 2 * sizeof(float));

    glClearColor(0.5, 0.8, 0.2, 1.0);

    float previousTime = glfwGetTime();

    while (!glfwWindowShouldClose(window))
    {
        float time = glfwGetTime();
        float dt = time - previousTime;
        previousTime = time;

        int w, h;
        glfwGetWindowSize(window, &w, &h);

        // en x, le [-1;1] <=> (0, w)
        // en y, le [-1;1] <=> (0, h)
        glViewport(0, 0, w, h);

        glClear(GL_COLOR_BUFFER_BIT);
        glBindVertexArray(vao);

        glEnable(GL_PROGRAM_POINT_SIZE);
        glDrawArrays(GL_POINTS, 0, sizeof(vertices) / sizeof(float) / 2);

        glfwSwapBuffers(window);
        glfwPollEvents();
    }

    glfwDestroyWindow(window);
    glfwTerminate();
    exit(EXIT_SUCCESS);
}

#define GLFW_INCLUDE_NONE
#include <GLFW/glfw3.h>
#include <glad/glad.h>
#include <glm/vec3.hpp>

#define GLM_ENABLE_EXPERIMENTAL
#include <glm/gtx/transform.hpp>

#include <algorithm>
#include <fstream>
#include <iostream>
#include <random>
#include <sstream>
#include <string>
#include <vector>

#include <glm/glm.hpp>
#include <glm/gtc/matrix_transform.hpp>
#include <glm/gtc/type_ptr.hpp>

#include <assimp/Importer.hpp>
#include <assimp/postprocess.h>
#include <assimp/scene.h>

#include "shader.h"

static void error_callback(int /*error*/, const char *description) {
  std::cerr << "Error: " << description << std::endl;
}

static void key_callback(GLFWwindow *window, int key, int /*scancode*/,
                         int action, int /*mods*/) {
  if (key == GLFW_KEY_ESCAPE && action == GLFW_PRESS)
    glfwSetWindowShouldClose(window, GLFW_TRUE);
}

void APIENTRY opengl_error_callback(GLenum source, GLenum type, GLuint id,
                                    GLenum severity, GLsizei length,
                                    const GLchar *message,
                                    const void *userParam) {
  std::cout << message << std::endl;
}

using glm::mat4;
using glm::vec4;

int main(void) {
  // Creation du context openGL avec glfw
  GLFWwindow *window;
  glfwSetErrorCallback(error_callback);

  if (!glfwInit())
    exit(EXIT_FAILURE);

  glfwWindowHint(GLFW_CONTEXT_VERSION_MAJOR, 4);
  glfwWindowHint(GLFW_CONTEXT_VERSION_MINOR, 6);
  glfwWindowHint(GLFW_OPENGL_DEBUG_CONTEXT, GLFW_TRUE);

  window = glfwCreateWindow(800, 800, "Gamagora triangle", NULL, NULL);

  if (!window) {
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
  if (!gladLoadGL()) {
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
  GLuint vbo, vao, vbo_color;

  // Attention à l'api avec des pointeurs
  // Souvent, en python par example:
  // [vbo] = glCreateBuffers(1)
  //
  // Nomenclature OpenGL:
  // gl[Create, Gen]XXXX
  // example:
  //    glCreateBuffers: nouvelle api, sans binds, avec des "Named" functions.
  //    glGenBuffers (pre Direct State Access "ancienne API", associée avec des
  //    "binds")
  glCreateBuffers(1, &vbo);
  glCreateBuffers(1, &vbo_color);
  std::vector<float> vertices;

  /*
  int nbPoints = 100;
  float pi = 3.14;
  float radius = 1;

  // Calcul des differents points sur un cerlce
  for (int i = 0; i < nbPoints; i++) {
    float angle = float(i) / nbPoints * 2 * pi;
    vertices.emplace_back(std::cos(angle) * radius);
    vertices.emplace_back(std::sin(angle) * radius);
  }
  */

  // Deux triangles
  /*
  vertices.push_back(-0.5);
  vertices.push_back(-0.5);
  vertices.push_back(-0.5);
  vertices.push_back(0.5);
  vertices.push_back(0.5);
  vertices.push_back(0.5);

  vertices.push_back(0.5);
  vertices.push_back(0.5);
  vertices.push_back(0.5);
  vertices.push_back(-0.5);
  vertices.push_back(-0.5);
  vertices.push_back(-0.5);
  */

  // Carré
  /*
  // Triangle strip
  vertices.push_back(-0.5);
  vertices.push_back(0.5);
  vertices.push_back(0.5);
  vertices.push_back(0.5);
  vertices.push_back(-0.5);
  vertices.push_back(-0.5);

  vertices.push_back(0.5);
  vertices.push_back(-0.5);

  std::vector<float> colors {
      1, 0, 0,
      0, 1, 0,
      0, 0, 1,
      1, 1, 1
  };
  */

  // The famous importer
  Assimp::Importer importer;
  auto path = "/home/guillaume/backup-guillaume/logo.obj";
  auto scene = importer.ReadFile(
      path, aiProcess_CalcTangentSpace | aiProcess_Triangulate |
                aiProcess_JoinIdenticalVertices | aiProcess_SortByPType |
                aiProcess_GenBoundingBoxes);

  if (nullptr == scene) {
    std::cout << importer.GetErrorString() << std::endl;
  }

  assert(scene->HasMeshes());
  assert(scene->mNumMeshes == 1);

  auto mesh = scene->mMeshes[0];
  std::cout << "Nombre de faces" << mesh->mNumFaces << std::endl;
  std::cout << "Nombre de vertices" << mesh->mNumVertices << std::endl;
  for (int faceNo = 0; faceNo < mesh->mNumFaces; ++faceNo) {
    auto face = mesh->mFaces[faceNo];
    assert(face.mNumIndices == 3);
    for (int idx = 0; idx < 3; ++idx) {
      auto v = mesh->mVertices[face.mIndices[idx]];
      vertices.push_back(v.x);
      vertices.push_back(v.y);
      vertices.push_back(v.z);
    }
  }

  std::cout << mesh->mAABB.mMax.x << std::endl;
  std::cout << mesh->mAABB.mMax.y << std::endl;

  std::cout << mesh->mAABB.mMin.x << std::endl;
  std::cout << mesh->mAABB.mMin.y << std::endl;

  std::cout << mesh->mAABB.mMax.x - mesh->mAABB.mMin.x << std::endl;
  std::cout << mesh->mAABB.mMax.y - mesh->mAABB.mMin.y << std::endl;
  std::vector<float> colors;

  // Copy des données dans le GPU
  glNamedBufferData(vbo, sizeof(vertices[0]) * vertices.size(), vertices.data(),
                    GL_DYNAMIC_DRAW);
  // glNamedBufferData(vbo_color, sizeof(colors[0]) * colors.size(),
  // colors.data(),
  //                   GL_DYNAMIC_DRAW);
  std::cout << "Nb points: " << vertices.size() / 3 << std::endl;

  // Bindings
  glCreateVertexArrays(1, &vao);
  const auto index = 0;
  const auto binding_point = 0;

  // Position
  // An active l'index (arbitraire, 0, c'est un choix) dans le vao
  glEnableVertexArrayAttrib(vao, index);
  // Associé à cet index, on aura des floats, par packet de 2
  glVertexArrayAttribFormat(vao, index, 3, GL_FLOAT, GL_FALSE, 0);
  // On va associer à l'index, le point "location" dans le shader
  glVertexArrayAttribBinding(vao, index, binding_point);

  // Associer au buffer
  glVertexArrayVertexBuffer(vao, binding_point, vbo, 0, 3 * sizeof(float));

  // Binding pour les couleurs
  const auto index_colors = 1;
  const auto binding_point_colors = 1;

  // Position
  // An active l'index_colors (arbitraire, 1, c'est un choix) dans le vao
  // glEnableVertexArrayAttrib(vao, index_colors);
  // Associé à cet index_colors, on aura des floats, par packet de 2
  glVertexArrayAttribFormat(vao, index_colors, 3, GL_FLOAT, GL_FALSE, 0);
  // On va associer à l'index_colors, le point "location" dans le shader
  glVertexArrayAttribBinding(vao, index_colors, binding_point_colors);

  // Associer au buffer
  glVertexArrayVertexBuffer(vao, binding_point_colors, vbo_color, 0,
                            3 * sizeof(float));

  glClearColor(0.5, 0.8, 0.2, 1.0);

  float previousTime = glfwGetTime();

  while (!glfwWindowShouldClose(window)) {
    float time = glfwGetTime();
    float dt = time - previousTime;
    previousTime = time;

    int w, h;
    glfwGetWindowSize(window, &w, &h);

    // en x, le [-1;1] <=> (0, w)
    // en y, le [-1;1] <=> (0, h)
    glViewport(0, 0, w, h);

    glClear(GL_COLOR_BUFFER_BIT | GL_DEPTH_BUFFER_BIT);
    glBindVertexArray(vao);

    {
      // scale /5
      mat4 scale = mat4(vec4(0.2, 0, 0, 0), vec4(0, 0.2, 0, 0),
                        vec4(0, 0, 0.2, 0), vec4(0, 0, 0, 1));

      mat4 translate = mat4(vec4(1, 0, 0, -0.5), vec4(0, 1, 0, 0),
                            vec4(0, 0, 1, 0), vec4(0, 0, 0, 1));

      float dw = time;
      mat4 rotation =
          mat4(vec4(cos(dw), -sin(dw), 0, 0), vec4(sin(dw), cos(dw), 0, 0),
               vec4(0, 0, 1, 0), vec4(0, 0, 0, 1));

      mat4 transformation = scale * rotation * translate;
      glProgramUniformMatrix4fv(program, 0, 1, false,
                                glm::value_ptr(transformation));

      glProgramUniform4f(program, 1, 0, 1, 0, 1);
      glDrawArrays(GL_TRIANGLES, 0, vertices.size() / 3);
    }

    { // scale /5
      mat4 scale = mat4(vec4(0.2, 0, 0, 0), vec4(0, 0.2, 0, 0),
                        vec4(0, 0, 0.2, 0), vec4(0, 0, 0, 1));

      mat4 translate = mat4(vec4(1, 0, 0, 0.5), vec4(0, 1, 0, 0),
                            vec4(0, 0, 1, 0), vec4(0, 0, 0, 1));

      float dw = time;
      mat4 rotation =
          mat4(vec4(cos(dw), -sin(dw), 0, 0), vec4(sin(dw), cos(dw), 0, 0),
               vec4(0, 0, 1, 0), vec4(0, 0, 0, 1));

      mat4 transformation = scale * rotation * translate;
      glProgramUniformMatrix4fv(program, 0, 1, false,
                                glm::value_ptr(transformation));

      glProgramUniform4f(program, 1, 1, 0, 0, 1);
      glDrawArrays(GL_TRIANGLES, 0, vertices.size() / 3);
    }

    glfwSwapBuffers(window);
    glfwPollEvents();
  }

  glfwDestroyWindow(window);
  glfwTerminate();
  exit(EXIT_SUCCESS);
}

/*
 *
 *
 *
 * En 3D, Draw indexé

- Nombre de faces: F
- Nombre de vertex par face: 3 (les 3 points)
- Nombre de coordonnée par vertex: 3 (x, y, z)
- Taille d'une coordonnée : sizeof(float): 4 octets

Stockage naif: F * 3 * 3 * 4 = F * 36


On observe qu'en moyenne, un vertex, est partagé par 6 faces


Nombre de vertex F * 3 / 6
Nombre de vertex uniques: F / 2

Stocker les vertex uniques: nb de vertex * nb coordonnée * taille d'une
coordonnée

Stockage des vertex uniques: F / 2 * 3 * 4 = F * 6.

Stocker les faces: 3 x numéro de vertex

Stockage des faces: 3 * F * sizeof(numero de vertex) = 3 * F * 4 = 12 * F.

Stockage: 18 * F

C'est deux fois plus petit que 36 * F


==> c'est plus efficace de stocker en mode "indexé", c'est à dire:

- Un tableau des vertex uniques
- Un tableau des indices de vertex pour toutes les faces

*/

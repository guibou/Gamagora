CXXFLAGS=-Iglad/include
CFLAGS=-Iglad/include
LDFLAGS=-lglfw -lGL

main: src/main.o src/shader.o glad/src/glad.o
	g++ src/main.o src/shader.o glad/src/glad.o ${LDFLAGS} -o main

glad/src/glad.o: glad/src/glad.c glad/include/glad/glad.h
src/main.o: src/main.cpp
src/shader.o: src/shader.h src/shader.cpp

clean:
	rm **/*.o
	rm main

CXXFLAGS=-Iglad/include -g -ggdb
CFLAGS=-Iglad/include -g -ggdb
LDFLAGS=-lglfw -lGL -lassimp -lm -lpthread -lX11

main: src/main.o src/shader.o glad/src/glad.o src/image.o
	g++ $^ ${LDFLAGS} -o $@

glad/src/glad.o: glad/src/glad.c glad/include/glad/glad.h
src/main.o: src/main.cpp
src/shader.o: src/shader.h src/shader.cpp
src/image.o: src/image.cpp

clean:
	rm **/*.o
	rm main

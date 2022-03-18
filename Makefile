

run:
	cd src && gcc main.c -lraylib -lglfw -lGL -lopenal -lm -pthread -ldl
	cd src && ./a.out


build:
	cd src && gcc main.c -lraylib -lglfw -lGL -lopenal -lm -pthread -ldl


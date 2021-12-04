all: one.sample one.input

one.sample: one
	./one < input.sample

one.input: one
	./one < input

one: one.c
	gcc -o one one.c
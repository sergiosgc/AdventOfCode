all: two.sample two.input

one.input: one
	./one < input
one.sample: one
	./one < input.sample
one: one.nim
	nim c --verbosity:0 one.nim
two.input: two
	./two < input
two.sample: two
	./two < input.sample
two: two.nim
	nim c --verbosity:0 two.nim

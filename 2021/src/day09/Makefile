all: one.sample one.input two.sample two.input

one.input: one
	./one < input
one.sample: one
	./one < input.sample
one: one.pp
	fpc one.pp
two.input: two
	./two < input
two.sample: two
	./two < input.sample
two: two.pp
	fpc two.pp
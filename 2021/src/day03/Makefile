all: one.sample

one.sample: one
	./one < input.sample

one.input: one
	./one < input

two.sample: two
	./two < input.sample

two.input: two
	./two < input

one: one.rs
	rustc one.rs

two: two.rs
	rustc two.rs




all: one.sample one two.sample two

one.sample: one.jl
	julia one.jl < input.sample
one: one.jl
	julia one.jl < input
two.sample: two.jl
	julia two.jl < input.sample
two: two.jl
	julia two.jl < input
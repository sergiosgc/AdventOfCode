one.sample: input.txt one.zig
	zig run one.zig < sample.txt

one: input.txt one.zig
	zig run one.zig < input.txt

two.sample: input.txt two.zig
	zig run two.zig < sample.txt

two: input.txt two.zig
	zig run two.zig < input.txt

input.txt: input.gpg
	gpg --decrypt --output=input.txt input.gpg

one.sample: input.txt one.nim
	nim compile --run one.nim < sample.txt

one: input.txt one.nim
	nim compile --run one.nim < input.txt

two.sample: input.txt two.nim
	nim compile --run two.nim < sample.txt

two: input.txt two.nim
	nim compile --run two.nim < input.txt

input.txt: input.gpg
	gpg --decrypt --output=input.txt input.gpg

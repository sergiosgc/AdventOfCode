all: one.sample one two.sample two

one.sample: one.bc
	(cat input.sample | tr "," "\n" | sed 's/\(.*\)/timer[\1] += 1/'; echo "i=80"; cat one.bc) | bc
one: one.bc
	(cat input| tr "," "\n" | sed 's/\(.*\)/timer[\1] += 1/'; echo "i=80"; cat one.bc) | bc
	 
two.sample: one.bc
	(cat input.sample | tr "," "\n" | sed 's/\(.*\)/timer[\1] += 1/'; echo "i=256"; cat one.bc) | bc
two: one.bc
	(cat input| tr "," "\n" | sed 's/\(.*\)/timer[\1] += 1/'; echo "i=256"; cat one.bc) | bc
	 
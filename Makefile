all: day7

day1:
	npx ts-node src/day1/index.ts
day2:
	npx ts-node src/day2/index.ts
day3:
	npx ts-node src/day3/index.ts
day4:
	npx ts-node src/day4/index.ts
day5: 
	npx ts-node src/day5/index.ts
day6:
	npx ts-node src/day6/index.ts
day7:
	tsc --downlevelIteration src/day7/index.ts
	node src/day7/index.js

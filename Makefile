all: day15

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
day8:
	tsc --downlevelIteration src/day8/index.ts
	node src/day8/index.js
day9:
	tsc --downlevelIteration src/day9/index.ts
	node src/day9/index.js
day10:
	tsc --downlevelIteration src/day10/index.ts
	node src/day10/index.js
day11:
	tsc --downlevelIteration src/day11/index.ts
	node src/day11/index.js
day12:
	tsc --downlevelIteration src/day12/index.ts
	node src/day12/index.js
day13:
	tsc --project tsconfig.json
	node src/day13/index.js
day14:
	tsc --project tsconfig.json
	node src/day14/index.js
day15:
	tsc --project tsconfig.json
	node src/day15/index.js

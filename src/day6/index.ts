import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput
  .split("\n")
  .map( (s:string) => s.split(")") )
  .reduce( (acc, orb) => { acc[orb[1]] = { 'parent': orb[0], 'orbits': null }; return acc; }, { 'COM': { 'parent': null, 'orbits': 0 }});

const input = prepareInput(readInput())

const goA = (input) => {
  let changed;
  do {
    changed = false;
    Object.keys(input).map( (planetName) => {
      if (input[planetName].orbits == null && input[input[planetName].parent].orbits != null) {
        input[planetName].orbits = input[input[planetName].parent].orbits + 1;
        changed = true;
      }
    });
  } while (changed);
  return Object.keys(input).reduce( (acc, planetName) => acc + input[planetName].orbits, 0);
}

const pathToCOM = (input, origin) => {
  let result = [[origin, 0]];
  while(result[0][0] != 'COM') result.unshift([ input[result[0][0]].parent, result[0][1]+1 ]);
  return result;
}

const goB = (input) => {
  let YOUpath = pathToCOM(input, 'YOU');
  let SANpath = pathToCOM(input, 'SAN');
  while (YOUpath[0][0] == SANpath[0][0]) {
    YOUpath.shift();
    SANpath.shift();
  }
  return YOUpath[0][1] + SANpath[0][1];
}
if (false) {
  console.log(goA(prepareInput("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L")));
  console.log(goB(prepareInput("COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN")));
   // test()
} else {
  console.time("Time")
  const resultA = goA(input)
  const resultB = goB(input)
  console.timeEnd("Time")

  console.log("Solution to part 1:", resultA)
  console.log("Solution to part 2:", resultB)
}
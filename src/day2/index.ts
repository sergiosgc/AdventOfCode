import { test, readInput } from "../utils/index"
import { exists } from "fs";

const prepareInput = (rawInput: string) => { let r = rawInput.split(',').map( (s:string):number => +s); r[1]=12; r[2]=2; return r; };

const input = prepareInput(readInput())

const ops = {
  1: function(pc, program) {
    program[program[pc+3]] = program[program[pc+1]] + program[program[pc+2]];
    return pc+4;
  },
  2: function(pc, program) {
    program[program[pc+3]] = program[program[pc+1]] * program[program[pc+2]];
    return pc+4;
  }
}

const goA = (input) => {
  let program = input.slice(0);
  let pc:number = 0;
  while (program[pc] != 99) {
    if (!ops[program[pc]]) return null;
    pc = ops[program[pc]](pc, program);
  }
  return program[0];
}

const goB = (input) => {
  for (let noun = 0; noun < 100; noun++) for (let verb = 0; verb < 100; verb++) {
    input[1] = noun;
    input[2] = verb;
    if (19690720 == goA(input)) return noun * 100 + verb;
  }
  return null;
}

/* Tests */

// test()
//console.log(goA([1,0,0,0,99]));
//console.log(goA([2,3,0,3,99]));
//console.log(goA([2,4,4,5,99,0]));
//console.log(goA([1,1,1,4,99,5,6,0,99]));
/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)

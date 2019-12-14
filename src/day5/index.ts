import { test, readInput } from "../utils/index"
import { exists } from "fs";

const prepareInput = (rawInput: string) => rawInput.split(',').map( (s:string):number => +s);

const input = prepareInput(readInput())

const parameterValue = (pc, program, index) => {
  let immediate = Math.trunc( program[pc] / 10 ** (index+1) ) % 10;
  return immediate ? program[pc + index] : program[program[pc + index]];
}

const ops = {
  1: function(pc, program, input, output) {
    program[program[pc+3]] = parameterValue(pc, program, 1) + parameterValue(pc, program, 2);
    return pc+4;
  },
  2: function(pc, program, input, output) {
    program[program[pc+3]] = parameterValue(pc, program, 1) * parameterValue(pc, program, 2);
    return pc+4;
  },
  3: function(pc, program, input, output) {
    program[program[pc+1]] = input.shift();
    return pc+2;
  },
  4: function(pc, program, input, output) {
    output.push(parameterValue(pc, program, 1));
    return pc+2;
  },
  5: function(pc, program, input, output) {
    if (parameterValue(pc, program, 1)) return parameterValue(pc, program, 2);
    return pc+3;
  },
  6: function(pc, program, input, output) {
    if (parameterValue(pc, program, 1) == 0) return parameterValue(pc, program, 2);
    return pc+3;
  },
  7: function(pc, program, input, output) {
    program[program[pc+3]] = parameterValue(pc, program, 1) < parameterValue(pc, program, 2) ? 1 : 0;
    return pc+4;
  },
  8: function(pc, program, input, output) {
    program[program[pc+3]] = parameterValue(pc, program, 1) == parameterValue(pc, program, 2) ? 1 : 0;
    return pc+4;
  },
}

const goA = (input) => {
  let program = input.slice(0);
  let vmInput = [1];
  let vmOutput = [];
  let pc:number = 0;
  while (program[pc] % 100 != 99) {
    if (!ops[program[pc] % 100]) return null;
    pc = ops[program[pc] % 100](pc, program, vmInput, vmOutput);
  }
  return vmOutput.pop();
}

const goB = (input) => {
  let program = input.slice(0);
  let vmInput = [5];
  let vmOutput = [];
  let pc:number = 0;
  while (program[pc] % 100 != 99) {
    if (!ops[program[pc] % 100]) return null;
    pc = ops[program[pc] % 100](pc, program, vmInput, vmOutput);
  }
  return vmOutput.pop();
}
if (false) {
  /* Tests */

  // test()
  console.log(goA([3,0,4,0,99]));
  console.log(goA([2,3,0,3,99]));
  console.log(goA([2,4,4,5,99,0]));
  console.log(goA([1,1,1,4,99,5,6,0,99]));
} else {
  /* Results */

  console.time("Time")
  const resultA = goA(input)
  const resultB = goB(input)
  console.timeEnd("Time")

  console.log("Solution to part 1:", resultA)
  console.log("Solution to part 2:", resultB)
}

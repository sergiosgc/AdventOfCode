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
const runProgram = (program, input) => {
  let localProgram = program.slice(0);
  let output = [];
  let pc:number = 0;
  while (localProgram[pc] % 100 != 99) {
    if (!ops[localProgram[pc] % 100]) return null;
    pc = ops[localProgram[pc] % 100](pc, localProgram, input, output);
  }
  return output;
}
const runSequence = (program, sequence) => {
  let result = [ null, sequence.slice(0) ];
  let signal = 0;
  while (sequence.length) {
    let input = [ sequence.shift(), signal ];
    signal = runProgram(program, input)[0];
  }
  result[0] = signal;
  return result;
}
function* permutationGenerator(pristineSet) {
  if (pristineSet.length == 0) {
    yield [];
    return;
  }
  for (let i=0; i<pristineSet.length; i++) {
    let baseArray = [ pristineSet[i] ];
    let subArray = pristineSet.slice(0);
    subArray.splice(i,1);
    let subGenerator = permutationGenerator(subArray);
    for (let subPermutation of subGenerator) yield baseArray.concat(subPermutation);
  }
}
const goA = (input) => {
  //  console.log(runSequence(input, [4,3,2,1,0]));
  let result = null;
  for (let sequence of permutationGenerator([4,3,2,1,0])) {
    let candidate = runSequence(input, sequence);
    result = result && result[0] > candidate[0] ? result : candidate;
  }
  return result[0];
}

const goB = (input) => {
}
if (false) {
  /* Tests */

  // test()
  console.log(goA([3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0]));
  console.log(goA([3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0]));
  console.log(goA([3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0]));



} else {
  /* Results */

  console.time("Time")
  const resultA = goA(input)
  const resultB = goB(input)
  console.timeEnd("Time")

  console.log("Solution to part 1:", resultA)
  console.log("Solution to part 2:", resultB)
}

import { test, readInput } from "../utils/index"
import { exists } from "fs";

const prepareInput = (rawInput: string) => rawInput.split(',').map( (s:string):number => +s);

const input = prepareInput(readInput())
const position = (pc, program, index) => {
  let relative = Math.trunc( program[pc] / 10 ** (index+1) ) % 10 == 2;
  return program[pc + index] + (relative ? program.relativeBase : 0);
}
const coalesce = (v, dflt) => v ? v : dflt;
const parameterValue = (pc, program, index) => {
  let immediate = Math.trunc( program[pc] / 10 ** (index+1) ) % 10 == 1;
  return coalesce(immediate ? program[pc + index] : program[position(pc, program, index)], 0);
}
const ops = {
  1: function(pc, program, input, output) {
    program[position(pc, program, 3)] = parameterValue(pc, program, 1) + parameterValue(pc, program, 2);
    return pc+4;
  },
  2: function(pc, program, input, output) {
    program[position(pc, program, 3)] = parameterValue(pc, program, 1) * parameterValue(pc, program, 2);
    return pc+4;
  },
  3: async function(pc, program, input, output) {
    program[position(pc, program, 1)] = await input.shift();
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
    program[position(pc, program, 3)] = parameterValue(pc, program, 1) < parameterValue(pc, program, 2) ? 1 : 0;
    return pc+4;
  },
  8: function(pc, program, input, output) {
    program[position(pc, program, 3)] = parameterValue(pc, program, 1) == parameterValue(pc, program, 2) ? 1 : 0;
    return pc+4;
  },
  9: function(pc, program, input, output) {
    program.relativeBase += parameterValue(pc, program, 1);
    return pc+2;
  }
}
const runProgram = async (program, input, output) => {
  let localProgram = program.slice(0);
  localProgram['relativeBase'] = 0;
  let pc:number = 0;
  while (localProgram[pc] % 100 != 99) {
    if (!ops[localProgram[pc] % 100]) return null;
    pc = await ops[localProgram[pc] % 100](pc, localProgram, input, output);
  }
  return output;
}
const goA = async (input) => {
  return await runProgram(input, [1], []);
}
const goB = async (input) => {
  return await runProgram(input, [2], []);
}

if (false) {
  /* Tests */

  // test()
  goA([109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]).then( r => console.log(r));
  goA([1102,34915192,34915192,7,4,7,99,0]).then( r => console.log(r));
  goA([104,1125899906842624,99]).then( r => console.log(r));


} else {
  /* Results */

  console.time("Time")
  goA(input).then( r => console.log("Solution to part 1;", r) );
  goB(input).then( r => console.log("Solution to part 2;", r) );
  console.timeEnd("Time")
}

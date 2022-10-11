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
class Robot {
  whitePanels = [];
  blackPanels = [];
  currentPos = [0,0];
  direction = [0, -1];
  state = 'paint'
  constructor(startOnWhite) {
    if (startOnWhite) this.whitePanels.push( [0,0] )
  }
  rotate(to) {
    this.direction = [ (to == 'left' ? 1 : -1) * this.direction[1], (to == 'left' ? -1 : 1) * this.direction[0] ]
  }
  paint(on) {
    if (on.filter( (panel) => panel[0] == this.currentPos[0] && panel[1] == this.currentPos[1]).length) return;
    on.push( this.currentPos.slice(0) )
    if (on == this.whitePanels) {
      this.blackPanels = this.blackPanels.filter( (panel) => panel[0] != this.currentPos[0] || panel[1] != this.currentPos[1] );
    } else {
      this.whitePanels = this.whitePanels.filter( (panel) => panel[0] != this.currentPos[0] || panel[1] != this.currentPos[1] );
    }
  }
  push(n:number) {
    if (this.state == 'paint') {
      this.paint(n ? this.whitePanels : this.blackPanels)
      this.state = 'move';
      return;
    }
    this.rotate( n == 0 ? 'left' : 'right' );
    this.currentPos[0] += this.direction[0];
    this.currentPos[1] += this.direction[1];
    this.state = 'paint';
  }
  shift() {
    let result = 
      this.whitePanels.filter( (panel) => panel[0] == this.currentPos[0] && panel[1] == this.currentPos[1]).length ? 1 : 0;
    return result;
  }
  resultA() {
    return this.whitePanels.length + this.blackPanels.length;
  }
}
const goA = async (input) => {
  let robot = new Robot(false);
  return (await runProgram(input, robot, robot)).resultA();
}
const goB = async (input) => {
  let robot = new Robot(true);
  await runProgram(input, robot, robot);
  let min = robot.whitePanels.reduce( (min, panel) => [
    min[0] !== null && min[0] < panel[0] ? min[0] : panel[0],
    min[1] !== null && min[1] < panel[1] ? min[1] : panel[1]
  ], [ null, null ]);
  let max = robot.whitePanels.reduce( (max, panel) => [
    max[0] !== null && max[0] > panel[0] ? max[0] : panel[0],
    max[1] !== null && max[1] > panel[1] ? max[1] : panel[1]
  ], [ null, null ]);
  max = [ max[0] - min[0], max[1] - min[1] ]
  let drawing = [];
  for (let y = 0; y<=max[1]; y++) drawing[y] = [];
  for (let y = 0; y<=max[1]; y++) for(let x = 0; x<=max[0]; x++) drawing[y].push(' ');
  return robot.whitePanels
    .map( (panel) => [ panel[0] - min[0], panel[1] - min[1] ])
    .reduce( (drawing, panel) => {
      drawing[panel[1]][panel[0]] = '█'; 
      return drawing;
    }, drawing)
    .map( (line) => line.join(''))
    .join("\n");
}

if (false) {

} else {
  /* Results */

  console.time("Time")
  goA(input).then( r => console.log("Solution to part 1;", r) );
  goB(input).then( r => console.log("Solution to part 2:\n"+ r) );
  console.timeEnd("Time")
}

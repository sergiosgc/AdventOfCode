import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput.split("\n").map( (s:string):number => +s);

const input = prepareInput(readInput())

const goA = (input) => {
  let map = input.reduce( function (acc, elm) { acc[elm] = true; return acc; }, Array.apply( null, new Array(2021)).map(() => false) );
  for (var i in input) if (input[i] != 1010 && map[2020 - input[i]]) return [input[i], 2020-input[i], input[i] * (2020-input[i])]; 
  return [ 1010, 1010, 1010*1010 ];
}

const goB = (input) => {
  input = input.sort( (a,b) => a-b );
  for (let i=0; i<input.length; i++) 
    for (let j=0; j<input.length && input[i] + input[j] <= 2020; j++) 
      for (let k=0; k<input.length && input[i] + input[j] + input[k] <= 2020; k++) 
        if (input[i] + input[j] + input[k] == 2020) return [input[i], input[j], input[k], input[i] * input[j] * input[k]];
}

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)

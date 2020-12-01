import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput.split("\n").map( (s:string):number => +s).sort( (a,b) => a-b );

const input = prepareInput(readInput())

const nSumProduct = function(n: number, target: number, input: Array<number>): number {
  for (let index: Array<number> = [ 0 ]; true; index[index.length - 1]++) {
    if (index[index.length - 1] == input.length) index.pop();
    else if (index.length < n) index.push(index[index.length - 1]);
    else if ( target == index.reduce( (acc, i) => acc + input[i], 0 )) return index.reduce( (acc, i) => acc * input[i], 1 );
  }
}

console.time("Time")
console.log("Solution to part 1:", nSumProduct(2, 2020, input))
console.log("Solution to part 2:", nSumProduct(3, 2020, input))
console.timeEnd("Time")
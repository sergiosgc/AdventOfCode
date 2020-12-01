import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput.split("\n").map( (s:string):number => +s).sort( (a,b) => a-b );

const input = prepareInput(readInput())

const nSumProduct = function(n: number, target: number, input: Array<number>): number {
  for (let indexStack: Array<number> = [ 0 ]; indexStack.length > 0; indexStack[indexStack.length - 1]++) {
    if (indexStack[indexStack.length - 1] == input.length) indexStack.pop();
    else if (indexStack.length < n) indexStack.push(indexStack[indexStack.length - 1]);
    else if ( target == indexStack.reduce( (acc, i) => acc + input[i], 0 )) return indexStack.reduce( (acc, i) => acc * input[i], 1 );
  }
  return null;
}

console.time("Time")
console.log("Solution to part 1:", nSumProduct(2, 2020, input))
console.log("Solution to part 2:", nSumProduct(3, 2020, input))
console.timeEnd("Time")
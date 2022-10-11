import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput.split("\n").map( (s:string):number => +s);

const input = prepareInput(readInput())

const goA = (input) => {
  return input
    .map( (mass:number):number => Math.trunc(mass/3) - 2)
    .filter( (fuel:number) => fuel > 0)
    .reduce( (acc:number, fuel: number) => acc + fuel, 0);
}

const recFuel = (mass: number) => {
  let fuel = Math.max(Math.trunc(mass/3) - 2);
  if (fuel <= 0) return 0;
  return fuel + recFuel(fuel);
}

const goB = (input) => {
  return input
    .map( recFuel )
    .filter( (fuel:number) => fuel > 0)
    .reduce( (acc:number, fuel: number) => acc + fuel, 0);
}

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)

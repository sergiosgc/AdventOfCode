import { test, readInput } from "../utils/index"
interface InputRow { min: number; max: number; password: string; letter: string }
// @ts-ignore
const input:Array<InputRow> = readInput().split("\n").map( (s:string) => {
  let parser = new RegExp('^(?<min>[0-9]+)-(?<max>[0-9]+) (?<letter>.): (?<password>.+)$');
  let result = s.match(parser);
  return result != null ? { min: result['groups'].min, max: result['groups'].max, letter: result['groups'].letter, password: result['groups'].password } : false;
}).filter( (x) => x ) || [];

const isPasswordValid = function(password) {
  let repeats = password.password.split('').filter( (char) => char == password.letter).length;
  return password.min <= repeats && repeats <= password.max;
}

console.time("Time")
console.log("Solution to part 1:", input.filter(isPasswordValid).length);
console.log("Solution to part 1:", input.filter( (p) => (p.password[p.min-1] == p.letter ? 1 : 0) ^ (p.password[p.max-1] == p.letter ? 1 : 0) ).length);
console.timeEnd("Time")
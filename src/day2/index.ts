import { test, readInput } from "../utils/index"
interface InputRow { min: number; max: number; password: string; letter: string }
const input:Array<InputRow> = readInput().split("\n").filter( (s) => s.length > 0 ).map( (s:string):InputRow => {
  let parser = new RegExp('^(?<min>[0-9]+)-(?<max>[0-9]+) (?<letter>.): (?<password>.+)$');
  let result = s.match(parser);
  return { min: +result['groups'].min, max: +result['groups'].max, letter: result['groups'].letter, password: result['groups'].password };
});

const isPasswordValid = function(password:InputRow) {
  let repeats = password.password.split('').filter( (char) => char == password.letter).length;
  return password.min <= repeats && repeats <= password.max;
}

console.time("Time")
console.log("Solution to part 1:", input.filter(isPasswordValid).length);
console.log("Solution to part 2:", input.filter( (p) => (p.password[p.min-1] == p.letter ? 1 : 0) ^ (p.password[p.max-1] == p.letter ? 1 : 0) ).length);
console.timeEnd("Time")
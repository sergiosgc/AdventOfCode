import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput.split('-').map( (s:string) => +s )

const input = prepareInput(readInput())

const meetsCriteria = (n:number) => {
  let digits = [];
  for (let i=5; i>=0; i--) digits.push( Math.trunc( n / (10**i) ) % 10 );
  return digits.reduce( (acc, d) => { return { "valid": acc.valid || acc.lastdigit == d,
          "lastdigit": d
        }
      }, { "valid": false, "lastdigit": null }).valid
    &&
      digits.reduce( (acc, d) => {
        return {
          "valid": acc.valid && (acc.lastdigit == null || acc.lastdigit <= d),
          "lastdigit": d
        }
      }, { "valid": true, "lastdigit": null }).valid;
}
const goA = (input) => {
  let count = 0;
  for (let i=input[0]; i<=input[1]; i++) if (meetsCriteria(i)) count++;
  return count;
}

const meetsElfCriteria = (n:number) => {
  let digits = [];
  for (let i=5; i>=0; i--) digits.push( Math.trunc( n / (10**i) ) % 10 );
  let state = 0;
  for (let i=0; i<6; i++) {
    switch (state) {
      case 0: state=1; break;
      case 1: state = digits[i-1] == digits[i] ? 2 : 1; break;
      case 2: 
        if (digits[i-1] != digits[i]) return true;
        state = 3; break;
      case 3: state = digits[i-1] == digits[i] ? 3 : 1; break;
    }
  }
  return state == 2;
}
const goB = (input) => {
  let count = 0;
  for (let i=input[0]; i<=input[1]; i++) if (meetsCriteria(i) && meetsElfCriteria(i)) count++;
  return count;
}

/* Tests */

// test()

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)

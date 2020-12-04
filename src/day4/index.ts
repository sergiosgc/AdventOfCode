import { test, readInput } from "../utils/index"
const input:Array<Map<string, string>> = readInput().trim().split("\n\n").map( (s) => s.trim().split("\n").map( (s) => s.split(" ") ).reduce( (acc, a) => acc.concat(a), [] ).reduce( function(acc,s) {
  let [k,v] = s.split(":", 2);
  acc.set(k,v);
  return acc;
}, new Map() )
);
const fieldValidationRegex = new Map([
    [ "iyr", /^201\d|2020$/ ],
    [ "eyr", /^202\d|2030$/ ],
    [ "byr", /^19[2-9]\d|200[0-2]$/ ],
    [ "hgt", /^((1[5-8]\d|19[0-3])cm|(59|6\d|7[0-6])in)$/ ],
    [ "hcl", /^#(?:[0-9a-f]{6})$/ ],
    [ "ecl", /^(?:amb|blu|brn|gry|grn|hzl|oth)$/],
    [ "pid", /^(?:[0-9]{9})$/ ],
]);
const isPassportValid = (p: Map<string, string>):boolean => Array.from(p.keys()).reduce( (acc, field) => acc && (!fieldValidationRegex.has(field) || fieldValidationRegex.get(field).test(p.get(field)) ), true );

console.time("Time")
console.log("Solution to part 1: ", input.filter( (m) => m.size == 8 || (m.size == 7 && !m.has("cid")) ).length);
console.log("Solution to part 2: ", input.filter( (m) => m.size == 8 || (m.size == 7 && !m.has("cid")) ).filter( isPassportValid ).length );
console.timeEnd("Time")
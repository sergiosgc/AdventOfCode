import { test, readInput } from "../utils/index"
const input:Array<Map<string, string>> = readInput().trim().split("\n\n").map( (s) => s.split("\n").map( (s) => s.split(" ") ).reduce( (acc, a) => acc.concat(a), [] ).reduce( function(acc,s) {
  let [k,v] = s.split(":", 2);
  acc.set(k,v);
  return acc;
}, new Map() )
);
const isPassportValid = function(p: Map<string, string>):boolean {
  let regex = new Map([
    [ "hgt", /^(?<value>[0-9]+)(?<unit>(?:in|cm))$/],
    [ "hcl", /^#(?:[0-9a-f]{6})$/ ],
    [ "ecl", /^(?:amb|blu|brn|gry|grn|hzl|oth)$/],
    [ "pid", /^(?:[0-9]{9})$/ ],
  ])
  for (let field of p.keys()) {
    switch (field) {
      case 'byr':
        if (+p.get(field) < 1920 || 2002 < +p.get(field)) return false;
        break;
      case 'iyr':
        if (+p.get(field) < 2010 || 2020 < +p.get(field)) return false;
        break;
      case 'eyr':
        if (+p.get(field) < 2020 || 2030 < +p.get(field)) return false;
        break;
      case "hgt":
      case "hcl":
      case "ecl":
      case "pid":
        if (!regex.get(field).test(p.get(field))) return false;
        if (field != "hgt") break;
        let match = p.get(field).match(regex.get(field));
        let [ unit, value ] = [ match['groups']['unit'], +match['groups']['value'] ];
        if ( unit == 'cm' && value < 150 || unit == 'cm' && 193 < value || unit == 'in' && value < 59 || unit == 'in' && 76 < value) return false;
        break;
    }
  }
  return true;
}
console.time("Time")
console.log("Solution to part 1: ", input.filter( (m) => m.size == 8 || (m.size == 7 && !m.has("cid")) ).length);
console.log("Solution to part 2: ", input.filter( (m) => m.size == 8 || (m.size == 7 && !m.has("cid")) ).filter( (p) => isPassportValid(p) ).length );
console.timeEnd("Time")
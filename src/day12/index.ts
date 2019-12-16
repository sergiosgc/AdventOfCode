import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput
  .split("\n")
  .map( (line) => line.match(/^<x=(?<x>.*), y=(?<y>.*), z=(?<z>.*)>/).groups )
  .map( (coords) => { return { "x": +coords.x, "y": +coords.y, "z": +coords.z, "velocity": { "x": 0, "y": 0, "z": 0 } }})

const input = prepareInput(readInput())

const goA = (moons, cycles) => {
  for (let i=0; i<cycles; i++) {
    moons.map( (anchor) => moons.map( (pulled) => ['x', 'y', 'z'].map( (axis) => pulled.velocity[axis] += anchor[axis] < pulled[axis] ? -1 : anchor[axis] > pulled[axis] ? 1 : 0 )))
    moons.map( (moon) => ['x', 'y', 'z'].map( (axis) => moon[axis] += moon.velocity[axis] ))
  }
  return moons
    .map( (moon) => (Math.abs(moon.x) + Math.abs(moon.y) + Math.abs(moon.z)) * (Math.abs(moon.velocity.x) + Math.abs(moon.velocity.y) + Math.abs(moon.velocity.z)) )
    .reduce( (sum, energy) => sum + energy, 0)

}
function gcd(x, y) {
 x = Math.abs(x);
 y = Math.abs(y);
 while(y) {
   var t = y;
   y = x % y;
   x = t;
 }
 return x;
}
function lcm(x, y) {
  if ((typeof x !== 'number') || (typeof y !== 'number')) 
   return false;
 return (!x || !y) ? 0 : Math.abs((x * y) / gcd(x, y));
}

const goB = (reference) => {
  return ["x", "y", "z"]
      .map( (equalityAxis) => {
        let moons = JSON.parse(JSON.stringify(reference))
        let count = 0;
        do {
          count++;
          moons.map( (anchor) => moons.map( (pulled) => ['x', 'y', 'z'].map( (axis) => pulled.velocity[axis] += anchor[axis] < pulled[axis] ? -1 : anchor[axis] > pulled[axis] ? 1 : 0 )))
          moons.map( (moon) => ['x', 'y', 'z'].map( (axis) => moon[axis] += moon.velocity[axis] ))
        } while (!moons
          .map( (moon, index) => moon[equalityAxis] == reference[index][equalityAxis] && moon.velocity[equalityAxis] == reference[index].velocity[equalityAxis])
          .reduce( (equal, local) => equal && local, true)
        );
        return count;
      })
      .reduce( (period, localPeriod) => lcm(period, localPeriod), 1)
}
if (false) {
  console.log(goA(prepareInput('\
<x=-1, y=0, z=2>\n\
<x=2, y=-10, z=-7>\n\
<x=4, y=-8, z=8>\n\
<x=3, y=5, z=-1>\
'), 10));
  console.log(goB(prepareInput('\
<x=-1, y=0, z=2>\n\
<x=2, y=-10, z=-7>\n\
<x=4, y=-8, z=8>\n\
<x=3, y=5, z=-1>\
')));

} else {
console.time("Time")
const resultA = goA(input, 1000)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)
}
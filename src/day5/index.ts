import { test, readInput } from "../utils/index"
const charBitmap = new Map([ ["F", 0 ], ["B", 1 ], [ "L", 0 ], [ "R", 1 ] ]);
const input:Set<number> = new Set(readInput().trim().split("\n").map( (s) => s.trim().split("").map( (c) => charBitmap.get(c) ).join("") ).map( (s) => parseInt(s, 2) ));
const emptyIsolatedSeats = function* (source:Set<number>) { 
  for (let i=1; i<1023; i++) if (source.has(i-1) && !source.has(i) && source.has(i+1)) yield i;
}
console.time("Time")
console.log("Solution to part 1: ", Math.max(...input));
console.log("Solution to part 2: ", Array.from(emptyIsolatedSeats(input)).filter( (id) => id > 7 && id < 1016 ).pop() );
console.timeEnd("Time")
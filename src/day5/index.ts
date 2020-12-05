import { test, readInput } from "../utils/index"
const charBitmap = new Map([ ["F", 0 ], ["B", 1 ], [ "L", 0 ], [ "R", 1 ] ]);
const input:Array<number> = readInput().trim().split("\n").map( (s) => s.trim().split("").map( (c) => charBitmap.get(c) ).join("") ).map( (s) => parseInt(s, 2) ).sort( (a,b) => a-b );
const emptyIsolatedSeats = (source:Array<number>) => Array.from(source.keys()).filter( (k) => k - 1 in source).map( (k) => source[k] - source[k-1] == 2 ? source[k]-1 : null ).filter( (seat) => seat != null );
console.time("Time")
console.log("Solution to part 1: ", Math.max(...input));
console.log("Solution to part 2: ", emptyIsolatedSeats(input).filter( (id) => id > 7 && id < 1016 ).pop() );
console.timeEnd("Time")
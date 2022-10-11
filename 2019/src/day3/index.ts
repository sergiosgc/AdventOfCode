import { test, readInput } from "../utils/index"

const delta = { 
  'U': { "x": 0, "y": 1 },
  'D': { "x": 0, "y": -1 },
  'L': { "x": -1, "y": 0 },
  'R': { "x": 1, "y": 0 }
};
const prepareInput = (rawInput: string) => rawInput
  .split("\n")
  .map( (wire:string) => wire
    .split(",")
    .map( (movement:string) => { return { "direction": movement[0], "steps": +movement.slice(1) } } )
    .reduce( (segments, movement) => { 
      let pos = segments[segments.length - 1];
      segments.push( {
        "start": pos.finish,
        "finish": {
          "x": pos.finish.x + delta[movement.direction].x * movement.steps,
          "y": pos.finish.y + delta[movement.direction].y * movement.steps,
          "distance": pos.finish.distance + movement.steps
        }
      });
      return segments;
    }, [ { 
        "start": { "x": 0, "y": 0, "distance": 0 }, 
        "finish": { "x": 0, "y": 0, "distance": 0} 
       } ] 
    )
  );

const input = prepareInput(readInput())
const intersect = (segmentA, segmentB) => {
  let xRange = [
    Math.max( Math.min(segmentA.start.x, segmentA.finish.x), Math.min(segmentB.start.x, segmentB.finish.x) ),
    Math.min( Math.max(segmentA.start.x, segmentA.finish.x), Math.max(segmentB.start.x, segmentB.finish.x) ),
  ];
  let yRange = [
    Math.max( Math.min(segmentA.start.y, segmentA.finish.y), Math.min(segmentB.start.y, segmentB.finish.y) ),
    Math.min( Math.max(segmentA.start.y, segmentA.finish.y), Math.max(segmentB.start.y, segmentB.finish.y) ),
  ];
  let result = [];
  for (let x = xRange[0]; x <= xRange[1]; x++) for (let y = yRange[0]; y <= yRange[1]; y++) result.push({
    "x": x,
    "y": y,
    "distance": 
      segmentA.start.distance + Math.abs(x - segmentA.start.x) + Math.abs(y - segmentA.start.y) + 
      segmentB.start.distance + Math.abs(x - segmentB.start.x) + Math.abs(y - segmentB.start.y)
  });
  return result;
}
const segmentsToIntersections = (input) => {
  return input[0]
      .map( (segmentA) => input[1].map( (segmentB) => intersect(segmentA, segmentB) ).filter( (intersections) => intersections.length > 0 ) ) 
      .reduce( (acc, arr) => acc.concat(arr), [])
      .reduce( (acc, arr) => acc.concat(arr), [])
      .filter( (intersection) => intersection.x != 0 || intersection.y != 0 );
}
const goA = (input) => {
    return segmentsToIntersections(input)
      .map( (intersection) => Math.abs(intersection.x) + Math.abs(intersection.y) )
      .reduce( (min, distance) => min && min < distance ? min : distance, false);
}

const goB = (input) => {
    return segmentsToIntersections(input)
      .reduce( (min, intersection) => min && min < intersection.distance ? min : intersection.distance, false);
}

/* Tests */
//console.log(goB(prepareInput("R8,U5,L5,D3\nU7,R6,D4,L4")));
//console.log(goB(prepareInput("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83")));

// test()

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:", resultB)

import { readInput } from "../utils/index"
class Coord { 
  x: number;
  y: number;
  constructor(x: number, y: number) { this.x = x; this.y = y; }
  equals(b: Coord) { return this.x === b.x && this.y === b.y; }
  incrementWithOverflowX(x: number, y: number, size: Coord) { this.x = (this.x + x) % size.x; this.y += y;}
  readMap(map: Array<Array<boolean>>) { return map[this.y][this.x]; }
}
const input:Array<Array<boolean>> = readInput().split("\n").filter( (s) => s.length > 0 ).map( (s:string):Array<boolean> => s.split('').map( (c) => c == "#" ) );

const traverseMap = function*(map: Array<Array<boolean>>, deltaX: number, deltaY: number) {
  let mapSize:Coord = new Coord(map[0].length, map.length);

  for (let pos:Coord = new Coord(0, 0); pos.y < mapSize.y; pos.incrementWithOverflowX(deltaX, deltaY, mapSize)) yield { pos: pos, tree: pos.readMap(map) };
}
const treeCountOnTraversal = (map: Array<Array<boolean>>, deltaX: number, deltaY: number) => Array.from(traverseMap(map, deltaX, deltaY)).filter( (p) => p.tree ).length;

console.time("Time")
console.log("Solution to part 1:", treeCountOnTraversal(input, 3, 1))
console.log("Solution to part 2:", [ 
  [1, 1],
  [3, 1],
  [5, 1],
  [7, 1],
  [1, 2],
].map( (delta) => treeCountOnTraversal(input, delta[0], delta[1])).reduce( (acc, x) => acc * x, 1));
console.timeEnd("Time")

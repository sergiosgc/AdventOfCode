import { readInput } from "../utils/index"
const inputA:Array<Array<string>> = readInput(false).trim().split("\n").map( (l) => l.trim().split("") )
const inputB:Array<Array<string>> = readInput(false).trim().split("\n").map( (l) => l.trim().split("") )
class Coord { 
    x: number; 
    y: number;
    value: string;
    public constructor(init?:Partial<Coord>, input?:Array<Array<string>>) {
        Object.assign(this, init); 
        if (input && init) this.read(input);
    }
    read(input:Array<Array<string>>):Coord {
        this.value = this.x != null && this.y != null && 0 <= this.y && this.y < input.length && 0 <= this.x && this.x < input[0].length ? input[this.y][this.x] : null;
        return this;
    }
}
const printSeating = (seating) => seating.map (row => { row.map( (value) => process.stdout.write(value)); process.stdout.write("\n") })
const adjacentA = function* (state, x, y) {
    for (let dx = -1; dx <= 1; dx++) for (let dy = -1; dy <= 1; dy++) if (dx != 0 || dy !=0) yield new Coord({ x: x+dx, y: y+dy}, state)
}
const adjacentB = function* (state, x, y) {
    let dm = [];
    for (let dx = -1; dx <= 1; dx++) for (let dy = -1; dy <= 1; dy++) if (dx != 0 || dy != 0) dm.push([dx,dy])
    for (let multiplier of dm) {
        let delta = 1;
        let coord:Coord = new Coord();
        do {
            coord.x = x + multiplier[0] * delta;
            coord.y = y + multiplier[1] * delta;
            coord.read(state);
            delta++;
        } while (coord.value == ".")
        if (coord.value != null) yield new Coord(coord, state)
    }
}
const iterateSeating = function(state: Array<Array<string>>,adjacent,occupiedThreshold) {
    let coordmap = state.map( (row, y) => row.map( (value, x) => new Coord({ x: x, y: y}, state) )).reduce( (acc, row) => acc.concat(row), [] )
    // @ts-ignore
    let occupy = coordmap.filter( (c) => c.value == "L" ).filter( (c) => 0 == Array.from(adjacent(state, c.x, c.y)).filter( (a) => a.value == "#" ).length )
    // @ts-ignore
    let vacate = coordmap.filter( (c) => c.value == "#" ).filter( (c) => occupiedThreshold <= Array.from(adjacent(state, c.x, c.y)).filter( (a) => a.value == "#" ).length )
    occupy.map( (c) => state[c.y][c.x] = "#" )
    vacate.map( (c) => state[c.y][c.x] = "L" )
    return occupy.length != 0 || vacate.length != 0
}
while(iterateSeating(inputA,adjacentA,4));
console.log( inputA.map( (row, y) => row.filter( (value) => value == "#" ).reduce( (count, row) => count += row.length, 0) ).reduce( (count, rowCount) => count += rowCount, 0) )
while(iterateSeating(inputB,adjacentB,5));
console.log( inputB.map( (row, y) => row.filter( (value) => value == "#" ).reduce( (count, row) => count += row.length, 0) ).reduce( (count, rowCount) => count += rowCount, 0) )
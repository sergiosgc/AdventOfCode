import { readInput } from "../utils/index"
class Movement { 
    action:string;
    argument:number;
    public constructor(init?: Partial<Movement>) { Object.assign(this, init); }
    move(ship:Ship):Ship { this["move" + this.action](ship); return ship; }
    moveN(ship:Ship) { ship.position.y -= this.argument; }
    moveS(ship:Ship) { ship.position.y += this.argument; }
    moveE(ship:Ship) { ship.position.x += this.argument; }
    moveW(ship:Ship) { ship.position.x -= this.argument; }
    moveL(ship:Ship) { ship.direction = (ship.direction + 360 - this.argument) % 360; }
    moveR(ship:Ship) { ship.direction = (ship.direction + this.argument) % 360; }
    moveF(ship:Ship) { this["move" + { 0: "N", 90: "E", 180: "S", 270: "W"}[ship.direction]](ship); }
    
    moveRTFM(ship:Ship):Ship { this["moveRTFM" + this.action](ship); return ship; }
    moveRTFMN(ship:Ship) { ship.waypoint.y -= this.argument; }
    moveRTFMS(ship:Ship) { ship.waypoint.y += this.argument; }
    moveRTFME(ship:Ship) { ship.waypoint.x += this.argument; }
    moveRTFMW(ship:Ship) { ship.waypoint.x -= this.argument; }
    moveRTFML(ship:Ship) { for (let i=0; i < (360 - this.argument) / 90; i++) ship.waypoint = { x:-ship.waypoint.y, y: ship.waypoint.x } }
    moveRTFMR(ship:Ship) { for (let i=0; i < this.argument / 90; i++) ship.waypoint = { x:-ship.waypoint.y, y: ship.waypoint.x } }
    moveRTFMF(ship:Ship) { ship.position = { x: ship.position.x + this.argument * ship.waypoint.x, y: ship.position.y + this.argument * ship.waypoint.y } }
}
class Ship { direction:number = 90; position:{x:number,y:number} = {x: 0, y: 0}; waypoint:{x:number,y:number} = {x: 10, y: -1} }
const input:Array<Movement> = readInput(false).trim().split("\n").map( (l) => new Movement({action: l[0], argument: +l.substring(1)}) )
console.log("Solution to part 1:", ( (ship) => Math.abs(ship.position.x) + Math.abs(ship.position.y) )(input.reduce( (ship, movement) => movement.move(ship), new Ship())) );
console.log("Solution to part 2:", ( (ship) => Math.abs(ship.position.x) + Math.abs(ship.position.y) )(input.reduce( (ship, movement) => movement.moveRTFM(ship), new Ship())) );
import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput
  .split("\n")
  .map( (line) => line.split('') )
  .map( (line, y) => line.map ( (cell, x) => { return { "x": x, "y": y, "asteroid": cell == '#' } } ) )
  .reduce( (acc, line) => acc.concat(line), [])
  .filter( (cell) => cell.asteroid )
  .map( (cell) => { return { "x": cell.x, "y": cell.y }} )



const input = prepareInput(readInput())

const goA = (input) => {
  return input
    .map( (base) => { return {
      "x": base.x,
      "y": base.y,
      "visible": input
        .filter( (asteroid) => asteroid.x != base.x || asteroid.y != base.y )
        .map( (asteroid) => Math.atan2(asteroid.y - base.y, asteroid.x - base.x) )
        .filter( (value, index, self) => self.indexOf(value) === index)
        .length
    }})
    .reduce( (min, res) => min && min.visible > res.visible ? min : res, null )
}

const goB = (input) => {
  let base = goA(input)
  return input
    .filter( (asteroid) => asteroid.x != base.x || asteroid.y != base.y )
    .map( (asteroid) => { return { 
      "x": asteroid.x,
      "y": asteroid.y,
      "angle": Math.atan2(asteroid.y - base.y, asteroid.x - base.x),
      "distance": (asteroid.y - base.y)**2 + (asteroid.x - base.x)**2
    }} )
    .map( (asteroid) => { 
      asteroid.angle += (Math.PI / 2)
      asteroid.angle += asteroid.angle < 0 ? 2 * Math.PI : 0
      return asteroid;
    })
    .sort( (a,b) => a.angle < b.angle ? -1 : a.angle > b.angle ? 1 : a.distance < b.distance ? -1 : a.distance > b.distance ? 1 : 0 )
    .map ( (asteroid, index, all) => {
      asteroid.depth = index == 0 || all[index-1].angle != asteroid.angle ? 0 : all[index-1].depth + 1;
      return asteroid;
    })
    .reduce( (acc, asteroid) => {
      acc[asteroid.depth] = acc[asteroid.depth] ? acc[asteroid.depth] : [];
      acc[asteroid.depth].push(asteroid);
      return acc;
    }, [])
    .reduce( (acc, depth) => acc.concat(depth) )[199]
}

if (false) {
//  console.log(goA(prepareInput(".#..#\n.....\n#####\n....#\n...##")));
//  console.log(goA(prepareInput('\
//......#.#.\n\
//#..#.#....\n\
//..#######.\n\
//.#.#.###..\n\
//.#..#.....\n\
//..#....#.#\n\
//#..#....#.\n\
//.##.#..###\n\
//##...#..#.\n\
//.#....####')));
//  console.log(goA(prepareInput('\
//#.#...#.#.\n\
//.###....#.\n\
//.#....#...\n\
//##.#.#.#.#\n\
//....#.#.#.\n\
//.##..###.#\n\
//..#...##..\n\
//..##....##\n\
//......#...\n\
//.####.###.')));
//  console.log(goA(prepareInput('\
//.#..#..###\n\
//####.###.#\n\
//....###.#.\n\
//..###.##.#\n\
//##.##.#.#.\n\
//....###..#\n\
//..#.#..#.#\n\
//#..#.#.###\n\
//.##...##.#\n\
//.....#.#..')));
  console.log(goA(prepareInput('\
.#..##.###...#######\n\
##.############..##.\n\
.#.######.########.#\n\
.###.#######.####.#.\n\
#####.##.#.##.###.##\n\
..#####..#.#########\n\
####################\n\
#.####....###.#.#.##\n\
##.#################\n\
#####.##.###..####..\n\
..######..##.#######\n\
####.##.####...##..#\n\
.#####..#.######.###\n\
##...#.##########...\n\
#.##########.#######\n\
.####.#.###.###.#.##\n\
....##.##.###..#####\n\
.#.#.###########.###\n\
#.#.#.#####.####.###\n\
###.##.####.##.#..##')));
  console.log(goB(prepareInput('\
.#..##.###...#######\n\
##.############..##.\n\
.#.######.########.#\n\
.###.#######.####.#.\n\
#####.##.#.##.###.##\n\
..#####..#.#########\n\
####################\n\
#.####....###.#.#.##\n\
##.#################\n\
#####.##.###..####..\n\
..######..##.#######\n\
####.##.####...##..#\n\
.#####..#.######.###\n\
##...#.##########...\n\
#.##########.#######\n\
.####.#.###.###.#.##\n\
....##.##.###..#####\n\
.#.#.###########.###\n\
#.#.#.#####.####.###\n\
###.##.####.##.#..##')));
} else {
  console.time("Time")
  const resultA = goA(input)
  const resultB = goB(input)
  console.timeEnd("Time")

  console.log("Solution to part 1:", resultA)
  console.log("Solution to part 2:", resultB)
}
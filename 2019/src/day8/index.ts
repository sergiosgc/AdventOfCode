import { test, readInput } from "../utils/index"

const prepareInput = (rawInput: string) => rawInput
  .match(/.{25}/g)
  .map( (line) => line
    .split('')
    .map( (n:string):number => +n ))
  .reduce( (layers, line) => {
    if (layers[layers.length-1].length == 6) layers.push([]);
    layers[layers.length-1].push(line);
    return layers;
  }, [[]])

const input = prepareInput(readInput())

const goA = (input) => {
  return input
    .map( (layer) => layer.reduce( (acc, line) => acc.concat(line), []))
    .map( (layer) => [
      layer.filter( pixel => pixel == 0).length ,
      layer.filter( pixel => pixel == 1).length * layer.filter( pixel => pixel == 2).length 
    ])
    .reduce( (min, layer) => min && min[0] < layer[0] ? min : layer, null )[1]
}

const goB = (input) => {
  let image = []
  for (let y=0; y<6; y++) image.push([]);
  for (let y=0; y<6; y++) for (let x=0; x<25; x++) image[y].push(2);
  for (let layer=0; layer < input.length; layer++) for (let y=0; y<6; y++) for (let x=0; x<25; x++) if ( image[y][x] == 2 ) image[y][x] = input[layer][y][x];

  return image.map( (line) => line.join('') ).join("\n").replace(/1/g, '█').replace(/0/g, ' ')
}

/* Tests */

// test()

/* Results */

console.time("Time")
const resultA = goA(input)
const resultB = goB(input)
console.timeEnd("Time")

console.log("Solution to part 1:", resultA)
console.log("Solution to part 2:\n" + resultB)

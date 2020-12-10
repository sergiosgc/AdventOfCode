import { readInput } from "../utils/index"
const input:Array<number> = readInput(false).trim().split("\n").map( (l) => +l ).sort( (a,b) => a-b )
const fillArrangementCount = function(data) {
    let counts = data.map( n => 0 )
    counts[counts.length - 1] = 1;
    for (let i = data.length - 2; i >= 0; i--) counts[i] = data.slice(i+1)
        .map ( (n, index) => n < data[i] + 4 ? index + i + 1 : null ) /* reachable indexes */
        .filter( n => n )
        .map( index => counts[index] )
        .reduce( (acc, n) => acc + n )
    return counts[0];
}

console.log( "Solution to part 1:", (hist => hist[1] * hist[3])(
    [0].concat(input, 3+Math.max(...input))
        .reduce( (acc, n) => { acc[acc.length-1][1] = n; acc.push([n]); return acc}, [[0]] )
        .filter( diff => diff.length == 2 && (diff[0] != 0 || diff[1] != 0))
        .map( diff => diff[1] - diff[0] )
        .reduce( (hist, diff) => { hist[diff]++; return hist }, [0,0,0,0])
    )
);
console.log( "Solution to part 2:", fillArrangementCount([0].concat(input, 3+Math.max(...input))) );
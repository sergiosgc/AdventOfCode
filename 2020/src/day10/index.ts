import { readInput } from "../utils/index"
const input:Array<number> = readInput(false).trim().split("\n").map( (l) => +l ).sort( (a,b) => a-b )
const arrangementCount = function(data) {
    let counts = Array(data.length);
    counts[counts.length - 1] = 1;
    for (let i = data.length - 2; i >= 0; i--) counts[i] = data.slice(i+1)
        .filter( n => n < data[i] + 4 ) /* reachable */
        .map ( (n, index) => counts[index + i + 1] ) 
        .reduce( (acc, n) => acc + n )
    return counts[0];
}

console.log( "Solution to part 1:", (hist => hist[1] * hist[3])(
    [0].concat(input, 3+Math.max(...input))
        .reduce( (acc, n) => { acc[acc.length-1][1] = n; acc.push([n]); return acc}, [[null]] )
        .filter( diff => diff.length == 2 && diff[0] != null )
        .map( diff => diff[1] - diff[0] )
        .reduce( (hist, diff) => { hist[diff]++; return hist }, [0,0,0,0])
    )
);
console.log( "Solution to part 2:", arrangementCount([0].concat(input, 3+Math.max(...input))) );

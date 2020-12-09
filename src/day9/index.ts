import { readInput } from "../utils/index"
const input:Array<number> = readInput().trim().split("\n").map( (l) => +l )
const findXMASInvalid = function(data:Array<number>, preambleSize:number) {
    mainloop: for (let n=preambleSize; n<data.length; n++) {
        for (let i=1; i<=preambleSize; i++) for (let j=i+1; j<=preambleSize; j++) if (data[n] == data[n-i] + data[n-j]) continue mainloop;
        return data[n];
    }
}
const findContiguousSumming = function(data:Array<number>, targetSum:number) {
    let i=0, j=0;
    while (targetSum != data.slice(i,j).reduce( (acc, n) => acc+n, 0 )) if (targetSum < data.slice(i,j).reduce( (acc, n) => acc+n, 0 )) { i++; j=i+1; } else j++;
    return Math.min(...data.slice(i,j)) + Math.max(...data.slice(i,j))
}
console.log("Solution to part 1:", findXMASInvalid(input, 25))
console.log("Solution to part 2:", findContiguousSumming(input, findXMASInvalid(input, 25)))
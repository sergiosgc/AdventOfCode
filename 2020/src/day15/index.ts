function *seq(start:number, end:number, step?:number) {
    step = step ? step : 1;
    for (let i=start; i<=end; i += step) yield i;
}

function *rambunctiousSeries(startingNumbers:number[]) {
    let lastTurn = new Map<Number, number>()
    let turn;
    let result;
    result = startingNumbers[0]
    yield result
    for (turn = 2; ; turn++) {
        let prevResult = result
        result = turn <= startingNumbers.length ? startingNumbers[turn - 1] : (lastTurn.has(result) ? turn-1 - lastTurn.get(result) : 0)
        lastTurn.set(prevResult, turn-1)
        yield result
    }
}
console.log( "Part 1:", ((series) => Array.from(seq(1,2020)).map( () => series.next().value ).reduce( (acc, v) => v ) )(rambunctiousSeries([0,3,6])) )
console.log( "Part 2:", ((series) => Array.from(seq(1,30000000)).map( () => series.next().value ).reduce( (acc, v) => v ) )(rambunctiousSeries([7,14,0,17,11,1,2])) )

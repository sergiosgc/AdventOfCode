process.stdin.setEncoding('utf-8');
coalesce = (v,onnull) => v == null ? onnull : v;
incr = (x, y) => ++board[y][x];
input = () => new Promise( (resolve, reject) => 
    process.stdin.on('readable', async () => 
        resolve(
            coalesce(process.stdin.read(), "")
                .split("\n").filter( s => s != "" )
                .map( s => s.split("").map( c => c - '0' )))));
let board = [];
function game() {
    let flashCount = 0;
    board = board.map( row => row.map( cell => cell + 1 ));
    let flashes = board
        .map( (row, y) => row.map( (cell, x) => cell > 9 ? [y,x] : null ))
        .flat()
        .filter( c => c);
    while(flashes.length) {
        flashCount += flashes.length;
        flashes = flashes
            .map( c => [ 
                [c[0]-1, c[1]-1], [c[0]-1, c[1]], [c[0]-1, c[1]+1], 
                [c[0]+1, c[1]-1], [c[0]+1, c[1]], [c[0]+1, c[1]+1], 
                [c[0], c[1]-1], [c[0], c[1]+1] ])
            .flat()
            .filter( c => c[0] >= 0 && c[0] < 10 && c[1] >= 0 && c[1] < 10)
            .filter( c => incr(c[1], c[0]) == 10 )
    }
    board = board.map( row => row.map( cell => cell > 9 ? 0 : cell ));
    return flashCount == 100;
}

async function main() {
    let i;
    board = await input();
    for (i = 1; !game(); i++);
    console.log(i);
}
main();

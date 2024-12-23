import sugar
import sequtils
import system
import coord2d
import vec2d
import std/tables

proc contains_word(board: Table[Coord2D, char], word: string, pos: Coord2D, dir: Vec2D): bool = 
    if word.len == 0: return true
    if not board.hasKey(pos) or board[pos] != word[0]: return false
    return board.contains_word(word[1..word.len-1], pos+dir, dir)


var input: Table[Coord2D, char] =
    stdin
    .lines
    .toSeq
    .pairs
    .toSeq
    .map( line => (
        line
        .val
        .pairs
        .toSeq
        .filter( ch => "XMAS".find(ch.val) != -1 )
        .map( ch => ( Coord2D(x: ch.key, y: line.key), ch.val ))
    ))
    .foldl( a.concat(b) )
    .toTable
echo(
    input
    .pairs
    .toSeq
    .filter( pair => pair[1] == 'X')
    .map( pair => 
        directions
        .toSeq
        .map( direction => input.contains_word("XMAS", pair[0], direction ))
        .filter( r => r)
    )
    .foldl( a.concat(b) )
    .len
)

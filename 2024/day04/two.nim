import sugar
import sequtils
import system
import coord2d
import vec2d
import rectangle2d
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
        .filter( ch => "MAS".find(ch.val) != -1 )
        .map( ch => ( Coord2D(x: ch.key, y: line.key), ch.val ))
    ))
    .foldl( a.concat(b) )
    .toTable
echo(
    input
    .pairs
    .toSeq
    .filter( pair => pair[1] == 'M')
    .map( pair => 
        directions
        .toSeq
        .map( direction => (pair[0], direction, input.contains_word("MAS", pair[0], direction )) )
        .filter( r => r[2])
        .map( r => bounding_box(r[0], r[0] + ("MAS".len()-1) * r[1]))
    )
    .foldl( a.concat(b) )
    .toSeq
    .foldl( (
        var result = a
        if a.hasKey(b):
            result.inc(b)
        else:
            result[b] = 1
        result
    ), CountTable[Rectangle2D]())
    .values
    .toSeq
    .filter( freq => freq == 2)
    .len
)

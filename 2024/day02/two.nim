import sugar
import sequtils
import strutils
import system

proc seqDeltas(s: seq[int]): seq[int] =
    var res: seq[int]
    for index in 0..(s.len-1):
        if index > 0:
            res.add(s[index] - s[index-1])
    return res

var input =
    stdin
    .lines
    .toSeq
    .map( line => line
                  .split(" ")
                  .map(part => part.strip())
                  .filter( part => part != "")
                  .map(parseInt)
    )
var deltas: seq[seq[seq[int]]]
for row in input:
    var candidates: seq[seq[int]]
    candidates.add(seqDeltas(row))
    for i in 0..row.len-1:
        var changedRow = row
        changedRow.delete(i)
        candidates.add(seqDeltas(changedRow))
    deltas.add(candidates)
echo(
    deltas
    .map( puzzle => # seq[seq[int]]
     puzzle.map( candidateDeltas => # seq[int]
        candidateDeltas
        .map( delta => (delta, abs(delta) > 0 and abs(delta) < 4))
        .foldl( (b[0], a[1] and b[1] and a[0]/abs(a[0]) == b[0]/abs(b[0]) ) )
     )
     .foldl( a or b[1], false)
    )
    .filter( a => a )
    .len()
)

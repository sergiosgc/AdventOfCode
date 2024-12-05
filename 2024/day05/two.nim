import sugar
import system
import strutils
import std/syncio
import sequtils
import tables
import algorithm

proc splitOn[T](s: openArray[T], op: proc (x: T): bool {.closure.}): (seq[T], seq[T]) =
    var split_index = 0
    for i in 0..s.len()-1:
        if s[i].len() == 0: split_index = i
    result = (s[0..split_index-1], s[split_index+1..s.len()-1])

type compareResult = enum lessThan, equal, unknown

proc compareWithCycleGuard(lessThanTable: Table[int, seq[int]], a: int, b: int, cycleStop: int): compareResult =
    if a == cycleStop: return unknown
    if a == b: return equal
    if not lessThanTable.hasKey(a): return unknown
    if lessThanTable[a].filter( candidate => candidate == b).len() > 0: return lessThan
    for candidate in lessThanTable[a]:
        if lessThanTable.compareWithCycleGuard(candidate, b, cycleStop) == lessThan: return lessThan
    return unknown

proc compare(lessThanTable: Table[int, seq[int]], a: int, b: int): compareResult =
    if a == b: return equal
    if not lessThanTable.hasKey(a): return unknown
    if lessThanTable[a].filter( candidate => candidate == b).len() > 0: return lessThan
    for candidate in lessThanTable[a]:
        if lessThanTable.compareWithCycleGuard(candidate, b, candidate) == lessThan: return lessThan
    return unknown

iterator combinations[T](s: openArray[T]): (T,T) =
    for i in 0..s.len()-1:
        for j in i+1..s.len()-1:
            yield (s[i], s[j])

var (inputRules, inputPages) = stdin
    .lines
    .toSeq
    .splitOn( line => line.len == 0 )
let lessThanTable = 
    inputRules
    .map( line => line.split("|").map( parseInt ).toSeq)
    .foldl( 
        (
            var res = a
            if a.hasKey(b[0]): res[b[0]].add(b[1]) else: res[b[0]] = @[b[1]]
            res
        )
        , Table[int, seq[int]]()
    )
let pages = 
    inputPages
    .map( line => line.split(",").map( parseInt ))
echo(
    pages
    .filter(
        line => line
                .combinations
                .toSeq
                .map( pagePair => lessThanTable.compare(pagePair[1], pagePair[0]) )
                .filter( comp => comp == compareResult.lessThan )
                .len() != 0
    )
    .map( line => (
        var sorted = line
        sorted.sort( (a,b) => (
            if a == b: return 0
            if lessThanTable.compare(a,b) == compareResult.lessThan: return -1
            if lessThanTable.compare(b,a) == compareResult.lessThan: return 1
            return 0
        ));
        sorted
    ))
    .map( line => line[((line.len()-1) / 2).int] )
    .foldl( a + b )
)


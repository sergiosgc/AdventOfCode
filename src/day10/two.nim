import std/strutils
import std/sequtils
import std/tables
import std/algorithm
import sugar

let 
    closemap = { ')': '(', ']': '[', '}': '{', '>': '<' }.toTable
    scoremap = { '(': 1, '[': 2, '{': 3, '<': 4 }.toTable

proc correction_score(line: seq[char], stack: seq[char]): int;
proc correction_score(line: seq[char]): int = return correction_score(line, @[])

proc read_input(): seq[seq[char]] =
    var f: File;
    discard f.open(0, fmRead)
    result = f.readAll().splitLines().map( (s) => (s & '.').items.toSeq() ) 

proc correction_score(line: seq[char], stack: seq[char]): int = 
    if len(line) == 1: 
        if len(stack) == 0: 
            return 0 
        else: 
            return stack.map( c => int(scoremap[c]) ).foldl( 5 * a + b )
    if closemap.hasKey(line[0]): 
        if stack[0] == closemap[line[0]]:
            return correction_score(line[1..^1], stack[1..^1])
        else:
            return 0
    else:
        return correction_score(line[1..^1], line[0..0] & stack)

proc median(arr: seq[int]): int = return arr[int((len(arr) - 1) / 2)]

echo(read_input().map(correction_score).filter( s => s != 0).sorted().median())

import std/strutils
import std/sequtils
import std/tables
import sugar

let 
    closemap = { ')': '(', ']': '[', '}': '{', '>': '<' }.toTable
    scoremap = { ')': 3, ']': 57, '}': 1197, '>': 25137 }.toTable

proc syntax_score(line: seq[char], stack: seq[char]): uint64;
proc syntax_score(line: seq[char]): uint64 = return syntax_score(line, @[])

proc read_input(): seq[seq[char]] =
    var f: File;
    discard f.open(0, fmRead)
    result = f.readAll().splitLines().map( (s) => (s & '.').items.toSeq() ) 

proc syntax_score(line: seq[char], stack: seq[char]): uint64 = 
    if len(line) == 1: return 0
    if closemap.hasKey(line[0]): 
        if stack[0] == closemap[line[0]]:
            return syntax_score(line[1..^1], stack[1..^1])
        else:
            return uint64(scoremap[line[0]])
    else:
        return syntax_score(line[1..^1], line[0..0] & stack)

echo(read_input().map(syntax_score).foldl(a+b))
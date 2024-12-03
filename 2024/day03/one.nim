import sugar
import sequtils
import strutils
import system
import std/re

var input =
    stdin
    .lines
    .toSeq
var regex = re"mul\((\d+),(\d+)\)"
echo(
    input
    .map( line => line
        .findAll(regex)
        .toSeq() 
        .map( s => (
            var matches: array[0..1, string]
            discard s.match(regex, matches)
            matches
            .map(parseInt)
        ))
        .map( val => val[0] * val[1])
        .foldl( a + b )
    )
    .foldl( a + b )
)

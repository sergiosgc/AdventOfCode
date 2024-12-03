import sugar
import sequtils
import strutils
import system
import std/re

var input =
    stdin
    .lines
    .toSeq
var regex = re"mul\((\d+),(\d+)\)|do\(\)|don't\(\)"
echo(
    input
    .map( line => line
        .findAll(regex)
        .toSeq() 
        .map( s => (
            case s:
                of "do()":
                    (0, 1)
                of "don't()":
                    (0, -1)
                else:
                    var matches: array[0..1, string]
                    discard s.match(regex, matches)
                    (matches
                     .map(parseInt)
                     .foldl( a * b )
                     , 0)
        ))
    )
    .foldl( a.concat(b) )
    .foldl( 
        (
            (
                case a[1]:
                    of 1: a[0] + b[0]
                    else: a[0]
            ),(
                case b[1]:
                    of 1: 1
                    of -1: 0
                    else: a[1]
            )
        )
        , (0,1)
    )[0]
)

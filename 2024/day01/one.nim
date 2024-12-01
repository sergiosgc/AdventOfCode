import sugar
import sequtils
import strutils
import system
import std/algorithm

var (leftList, rightList) = stdin
    .lines
    .toSeq
    .map( line => line
                  .split(" ", 2)
                  .map(part => part.strip())
                  .filter( part => part != "")
                  .map(parseInt)
    )
    .map( line => (line[0], line[1]) )
    .unzip
leftList.sort()
rightList.sort()
echo(
    leftList
    .zip(rightList)
    .toSeq
    .map( row => ( abs(row[0] - row[1]) ) )
    .foldl( a+b )
)
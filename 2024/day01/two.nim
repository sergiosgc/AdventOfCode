import sugar
import sequtils
import strutils
import system

var (leftList, rightList) = stdin
    .lines
    .toSeq
    .map( line => line
                  .split(" ", 2)
                  .map( part => part.strip() )
                  .filter( part => part != "" )
                  .map(parseInt)
    )
    .map( line => (line[0], line[1]) )
    .unzip
echo(
    leftList
    .map( left => (
        left * rightList.filter( right => left == right ).len()
    ))
    .foldl( a + b )
)
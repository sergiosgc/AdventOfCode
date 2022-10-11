open System
open System.Collections.Generic 

let display( l: HashSet<int*int>) = 
  let mutable maxX = 0
  let mutable maxY = 0
  for p in l do
    match p with 
    | (a,b) -> maxX <- Math.Max( a, maxX)
    match p with 
    | (a,b) -> maxY <- Math.Max( b, maxY)
  for y = 0 to maxY do
    for x = 0 to maxX do
      Console.Write(if l.Contains( (x,y) ) then "#" else ".")
    Console.WriteLine ""

let fold( dots: HashSet<int * int>, f: int * int ) = 
  let result = HashSet<int*int>()

  match f with 
    | (fx, fy) -> (
      for dot in dots do 
        match dot with 
        | (x,y) -> result.Add ( (Math.Abs ((Math.Abs (x-fx))-fx)), (Math.Abs ((Math.Abs (y-fy))-fy) ) )
        |> ignore
    )

  result

let mutable dots = HashSet(
  Seq.initInfinite (fun _ -> Console.ReadLine()) 
    |> Seq.takeWhile ((<>) "") 
    |> Seq.map( fun s -> int(s.Split(",")[0]), int(s.Split(",")[1]) ))

let folds = List(
  Seq.initInfinite (fun _ -> Console.ReadLine()) 
    |> Seq.takeWhile ((<>) null)
    |> Seq.map( fun s -> (if s.StartsWith("fold along x") then int(s.Split("=")[1]) else 0), (if s.StartsWith("fold along y") then int(s.Split("=")[1]) else 0) )
)

for f in folds do
  dots <- fold( dots, f)

display( dots )
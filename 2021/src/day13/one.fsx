open System
open System.Collections.Generic 

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

dots <- fold( dots, folds[0])
Console.WriteLine( List(dots).Count )



import coord2d

type Rectangle2D* = object of RootObj
  min*: Coord2D
  max*: Coord2D

proc `==`*(a: Rectangle2D, b: Rectangle2D): bool =
  result = a.min == b.min and a.max == b.max

proc bounding_box*(a: Coord2D, b: Coord2D): Rectangle2D =
  result = Rectangle2D( 
    min: Coord2D(x: a.x.min(b.x), y: a.y.min(b.y)),
    max: Coord2D(x: a.x.max(b.x), y: a.y.max(b.y))
  )
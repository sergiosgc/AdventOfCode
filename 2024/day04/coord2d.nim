import vec2d

type Coord2D* = object of RootObj
  x*: int
  y*: int

proc `+`*(a: Coord2D, b: Vec2D): Coord2D =
  result = Coord2D(x: a.x + b.x, y: a.y + b.y)

proc `+`*(a: Vec2D, b: Coord2D): Coord2D =
  result = b + a

proc `==`*(a: Coord2D, b: Coord2D): bool =
  result = a.x == b.x and a.y == b.y
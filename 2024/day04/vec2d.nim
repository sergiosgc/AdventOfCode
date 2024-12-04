type Vec2D* = object of RootObj
  x*: int
  y*: int

proc `+`*(a, b: Vec2D): Vec2D =
    result = Vec2D(x: a.x + b.x, y: a.y + b.y)

proc `-`*(a, b: Vec2D): Vec2D =
    result = Vec2D(x: a.x - b.x, y: a.y - b.y)

proc `*`*(a: Vec2D, b: int): Vec2D =
    result = Vec2D(x: a.x * b, y: a.y * b)

proc `*`*(a: int, b: Vec2D): Vec2D =
    result = b * a

proc `==`*(a: Vec2D, b: Vec2D): bool =
  result = a.x == b.x and a.y == b.y

iterator directions*(): Vec2D =
  let axis_directions = @[-1, 0, 1]
  for x in axis_directions:
    for y in axis_directions:
      if x == 0 and y == 0: continue
      yield Vec2D(x: x, y: y)
using Pipe: @pipe

mutable struct Coordinate 
    x::Int
    y::Int
end

struct Vent 
    from::Coordinate
    to::Coordinate
end

function input() 
    s = read(stdin, String)
    return @pipe (split(s, "\n", keepempty = false) 
        |> map( l -> match(r"(\d+),(\d+) -> (\d+),(\d+)", l), _ )
        |> map( m -> Vent( Coordinate(Base.parse(Int, m.captures[1]), Base.parse(Int, m.captures[2])), Coordinate(Base.parse(Int, m.captures[3]), Base.parse(Int, m.captures[4])) ), _)
    )
end
function expand_vent(v::Vent)
    result = Coordinate[]
    inc = Coordinate( v.from.x == v.to.x ? 0 : (v.to.x - v.from.x) / abs(v.to.x - v.from.x), v.from.y == v.to.y ? 0 : (v.to.y - v.from.y) / abs(v.to.y - v.from.y) )
    cursor = v.from
    while cursor.x != v.to.x || cursor.y != v.to.y
        push!(result, Coordinate(cursor.x, cursor.y))
        cursor.x += inc.x
        cursor.y += inc.y
    end
    push!(result, v.to)
    return result
end
@pipe (input()  
    |> map( expand_vent, _ )
    |> Iterators.flatten( _ )
    |> collect( _ )
    |> sort( _, lt = (a,b) -> a.x < b.x || (a.x == b.x && a.y < b.y) )
    |> map( c -> (1, c), _)
    |> reduce( function(acc, c) 
        if length(acc) > 0 && acc[1][2].x == c[2].x && acc[1][2].y == c[2].y
            acc[1] = (acc[1][1] + c[1], acc[1][2])
        else
            pushfirst!(acc, c)
        end
        return acc
    end, _, init=[])
    |> filter( t -> t[1] > 1, _)
    |> length(_)
    |> println(_) )

uses sysutils;

var 
    depth: array[1..100, 1..100] of integer;
    basin: array[1..100, 1..100] of integer;
    width: integer;
    height: integer;

procedure read_input();
var
    line: string;
    x: integer;
    y: integer;
begin
    y := 0;
    while not eof(input) do
    begin
        y := y + 1;
        readln(line);
        for x := 1 to Length(line) do
        begin
            depth[y,x] := Ord(line[x]) - Ord('0');
            basin[y,x] := 0;
        end;
    end;
    width := x;
    height := y;
end;

function low_point(y: integer; x: integer): boolean;
begin
    low_point := ((x = 1) or (depth[y,x-1] > depth[y,x])) and ((x = width) or (depth[y,x+1] > depth[y,x])) and ((y = 1) or (depth[y-1,x] > depth[y,x])) and ((y = height) or (depth[y+1,x] > depth[y,x]));
end;

procedure mark_basins();
var
    x: integer;
    y:integer;
    color: integer;
begin
    color := 0;
    for x := 1 to width do for y:= 1 to height do if low_point(y, x) then 
    begin
        color := color + 1;
        basin[y,x] := color;
    end;
end;

procedure flood_fill_basin(y: integer; x: integer; color: integer);
begin
    if ((depth[y,x] = 9) or (basin[y,x] <> 0)) then Exit;
    basin[y,x] := color;
    if (y > 1) then if (depth[y,x] < depth[y-1,x]) then flood_fill_basin(y-1, x, color);
    if (y < height) then if (depth[y,x] < depth[y+1,x]) then flood_fill_basin(y+1, x, color);
    if (x > 1) then if (depth[y,x] < depth[y,x-1]) then flood_fill_basin(y, x-1, color);
    if (x < width) then if (depth[y,x] < depth[y,x+1]) then flood_fill_basin(y, x+1, color);
end;

procedure print_three_largest_basins();
var
    x: integer;
    y: integer;
    basin_size: array[1..10000] of integer;
    color: integer;
begin
    for x := 1 to width do for y:= 1 to height do if basin[y,x] <> 0 then 
    begin
        color := basin[y,x];
        basin[y,x] := 0;
        flood_fill_basin(y, x, color);
    end;
    for x := 1 to 10000 do basin_size[x] := 0;
    for x := 1 to width do for y:= 1 to height do if basin[y,x] <> 0 then basin_size[basin[y,x]] := basin_size[basin[y,x]] + 1;
    for x := 1 to 10000-1 do for y := x+1 to 10000 do if basin_size[x] < basin_size[y] then begin
        color := basin_size[x];
        basin_size[x] := basin_size[y];
        basin_size[y] := color;
    end;
    writeln(basin_size[1] * basin_size[2] * basin_size[3]);
end;

begin
    read_input();
    mark_basins();
    print_three_largest_basins();
end.
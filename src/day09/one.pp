uses sysutils;

var 
    depth: array[1..100, 1..100] of integer;
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
        for x := 1 to Length(line) do depth[y,x] := Ord(line[x]) - Ord('0');
    end;
    width := x;
    height := y;
end;

function low_point(y: integer; x: integer): boolean;
begin
    low_point := ((x = 1) or (depth[y,x-1] > depth[y,x])) and ((x = width) or (depth[y,x+1] > depth[y,x])) and ((y = 1) or (depth[y-1,x] > depth[y,x])) and ((y = height) or (depth[y+1,x] > depth[y,x]));
end;

function risk_score(): integer;
var
    x: integer;
    y:integer;
    res: integer;
begin
    res := 0;
    for x := 1 to width do for y:= 1 to height do if low_point(y, x) then res := res + depth[y,x] + 1;
    risk_score := res;
end;

begin
    read_input();
    writeln(risk_score());
end.
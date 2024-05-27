program ejercicio_3;

const dimF = 10;

type
  vector = array [1..dimF] of integer;

procedure inciso_a(v: vector; dimL, i: integer);
begin
  for i:=1 to dimL do
    writeln(v[i]);
end;

procedure inciso_b(v:vector; dimL, i: integer);
begin
  for i:= dimL downto 1 do
    writeln(v[i]);
end;

procedure inciso_c(v: vector; dimL, i: integer);
begin
  for i:= (dimL DIV 2) downto 1 do
    writeln(v[i]);

  for i:=((dimL DIV 2)+1) to dimL do
    writeln(v[i]);
end;

procedure inciso_d(v: vector; x, y, i, dimL: integer);
begin
  if x >= y then
    for i:= x to y do
      writeln(v[i])
  else if y > x then
    for i:= y downto x do
      writeln(v[i])
end;

var
  dimL, i, x, y : integer;
  v: vector;

begin
  dimL:=0;
end.

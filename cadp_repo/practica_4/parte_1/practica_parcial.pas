program parcial;

type
  vec = array [1..10] of integer;

function max(v:vec; dimL: integer): integer;
var i, max, act: integer;
begin
  for i:=1 to dimL do begin
    if v[i] > max then
      max:=v[i];
  end;
end;


var 
  i: integer;
  v:vec;

begin
  for i in v do begin
    vec[i]:= 0;
  end;

  for i in v do
  begin
    writeln(vec[i]);
  end;
end.

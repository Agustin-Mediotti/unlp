program ProgramaVectores;

const
  fin = 0;
  dimF = 50;

type
  vector = array[1..dimF] of integer;

function rnd(min,max:integer):integer;
var num:integer;
begin
  num:=0;
  while (num < min) do
    num:=random(max);
  rnd:=num;
end;

procedure ImprimirVector(vec:vector; dimL:integer);
var i:integer;
begin
  for i:=1 to dimL do
    writeln(vec[i]);
end;

procedure CargarVector(var vec: vector; var dimL:integer; A, B: integer);
var num:integer;
begin
  dimL:=1;
  while num <> 0 do begin
    num:=rnd(A,B);
    if num <> 0 then begin
      vec[dimL]:=num;
      dimL:=dimL+1;
      end;
  end;
end;

var
  A,B,dimL:integer; vec:vector;
begin
  randomize;

  readln(A);
  readln(B);

  CargarVector(vec, dimL, A, B);
  ImprimirVector(vec, dimL);
end.

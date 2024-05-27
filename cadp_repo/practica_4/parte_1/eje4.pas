program ejercicio_4;

type
  vec = array[1..100] of integer;

function posicion(x:integer; v:vec):integer; // inciso a
var i:integer;
begin
  for i:=1 to 100 do
    if v[i] = x then
      posicion:= i;
  end;
  posicion:= -1;
end;

procedure intercambio(x,y:integer; var v:vec); // inciso b
var xpos, ypos:integer;
begin
  xpos:= posicion(x, v);
  ypos:= posicion(y, v);
  v[xpos]:= y;
  v[ypos]:= x;
end;

function sumaVector(v:vec):integer; // inciso c
var i, sum: integer;
begin
  sum:=0;
  for i:=1 to 100 do
    sum:= sum+v[i];
  sumaVector:=sum;
end;

function promedio(v:vec):real; // inciso d
begin
  promedio:= sumaVector(v) / 100;
end;

function elementoMaximo(v:vec):integer; // inciso e
var max, i: integer;
begin
  max:=-9999;
  for i:=1 to 100 do
    if v[i] > max then
      max:= v[i];
    elementoMaximo:=max;
end;

function elementoMinimo(v:vec):integer; // inciso f
var min, i: integer;
begin
  min:=9999;
  for i:=1 to 100 do
    if v[i] < min then
      min:= v[i];
    elementoMinimo:=min;
end;

begin
  writeln('hello');
end.

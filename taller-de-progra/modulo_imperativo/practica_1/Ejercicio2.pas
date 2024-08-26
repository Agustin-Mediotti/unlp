program Ejercicio_2;

const 
  FIN = -1;
  DIMF = 300;

type
  rango_oficinas = 1..DIMF;
  oficina = record
    cod: integer;
    dni: integer;
    val_exp: real;
  end;
  vector = array[rango_oficinas] of oficina;

procedure LeerOficina(var ofi:oficina);
begin
  readln(ofi.cod);
  ofi.dni:= random(150);
  ofi.val_exp:= random(20);
end;

procedure OrdenarSeleccion(var vec:vector; dimL:integer);
var
  x,i,aux:integer;
  item:oficina;
begin
  for x:=1 to dimL-1 do 
  begin
    aux:=x;
    for i:=x+1 to dimL do
      begin
        if vec[i].cod < vec[aux].cod then aux:=i;
      end;
  end;

  item:=vec[aux];
  vec[aux]:=vec[x];
  vec[x]:=item;
end;

procedure OrdenarInsercion(var vec: vector; dimL: integer);
var
  i,j:integer;
  act:oficina;
begin
  for i:=2 to dimL do
  begin
    act:=vec[i];
    j:=i-1;
    while(j>0) and (vec[j].cod > act.cod) do
      begin
        vec[j+1]:=vec[j];
        j:=j-1;
      end;
      vec[j+1]:= act;
  end;
end;

procedure CargarVector(var vec:vector; var dimL: integer);
var ofi:oficina;
begin
  LeerOficina(ofi);
  while ((ofi.cod <> FIN) and (dimL < DIMF)) do begin
    vec[dimL]:= ofi;
    LeerOficina(ofi);
    dimL:=dimL+1;
  end;
end;

var
  vec_ofi: vector; dimL: integer;
begin
  dimL:=0;
  CargarVector(vec_ofi, dimL);      {inciso a}
  OrdenarSeleccion(vec_ofi, dimL);  {inciso b}
  OrdenarInsercion(vec_ofi, dimL);  {inciso c}
end.

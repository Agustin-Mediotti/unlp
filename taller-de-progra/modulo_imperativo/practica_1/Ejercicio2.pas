program ejercicio2;

type
  rango_oficinas = 1..300;
  oficina = record
    cod: integer;
    dni: integer;
    val_exp: real;
  end;
  vector = array[rango_oficinas] of oficina;

procedure OrdenarVector(var vec; dimL:integer);
var i,j,pos,item:integer;
begin
 for i:=1 to dimL-1 do begin
  pos:=i;
  for j:=i+1 to dimL do
    if vec[j] < vec[pos] then pos:=j;

  item:=vec[pos];
  vec[pos]:=vec[i];
  vec[i]:=item;
 end;
end;

procedure LeerOficina(var oficina);
begin
  readln(oficina.cod);
  oficina.dni:= random(150);
  oficina.val_exp:= random(20);
end;

procedure CargarVector(var vec:vector; var dimL: integer);
var ofi:oficina;
begin
  LeerOficina(ofi);
  while ofi.cod <> -1 do begin
    vec[dimL] = ofi;
    LeerOficina(ofi);
    dimL:=dimL+1;
  end;
end;

var
  vec_ofi: vector; dimL: integer;
begin
  dimL:=0;
  CargarVector(vec_ofi, dimL);
  OrdenarVector(vec_ofi, dimL);
end.

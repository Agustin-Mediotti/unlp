program ejercicio_7;

procedure int_deconst (number:integer; var cant_dig: integer; var suma_dig: integer); { inciso a }
var
  i, aux: integer;
begin
  aux:=number;
  while (aux <> 0) do begin
    i:= aux MOD 10;
    cant_dig:= cant_dig+1;
    suma_dig:= suma_dig+i;
    aux:= aux DIV 10
  end;
end;

var
  num, cant_dig, suma_dig, total: integer;

begin {inciso b}
  suma_dig:=0;
  total:=0;
  while(suma_dig <> 10) do begin
    cant_dig:=0;
    suma_dig:=0;
    read(num);
    int_deconst(num, cant_dig, suma_dig);
    total:=total+cant_dig;
  end;
  writeln('cantidad total de digitos leidos: ', total);
end.

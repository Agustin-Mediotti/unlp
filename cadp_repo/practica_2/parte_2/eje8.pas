program ejercicio_8;

procedure redux_num (num: integer; var sum_par, cant_inpar: integer);
var
  i, aux: integer;
begin
  aux:=num;
  while (aux <> 0) do begin
    i:= aux MOD 10;
    if i MOD 2 = 0 then
      sum_par:= sum_par+i
    else
      cant_inpar:=cant_inpar+1;
    aux:= aux DIV 10;
  end;
  writeln('cantidad impares: ', cant_inpar, ' cantidad digitos pares: ', sum_par);
end;

var
  num, sum_par, cant_inpar: integer;

begin
  sum_par:=0;
  cant_inpar:=0;
  readln(num);
  while (num <> 12345) do begin { le saque un digito al ejercicio porque se va de rango de enteros de pascal }
    redux_num(num, sum_par, cant_inpar);
    readln(num);
  end;
end.

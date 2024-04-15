program ejercicio_2;

var
  num, i, mayor, posicion: integer;
begin
  num:=0;
  mayor:=0;
  posicion:=0;
  for i:=1 to 10 do begin
    read(num);
    if num > mayor then begin
      mayor:=num;
      posicion:=i;
    end;
  end;
  writeln('mayor fue: ', mayor, ' en la posicion ', posicion);
end.

program ejercicio_5;

function double(numA, numB: integer): boolean;
begin
  double:= numA*2 = numB;
end;

var
  pares, cantIsDouble, numA, numB: integer;

begin
  pares:=0;
  cantIsDouble:=0;
  read(numA);
  read(numB);
  while ((numA <> 0) and (numB <> 0)) do begin
    pares:=pares+1;
    read(numA);
    read(numB);
    if double(numA, numB) then
      cantIsDouble:=cantIsDouble+1;
  end;
  writeln('cantidad de pares: ', pares);
  writeln('cantidad de pares con numB doble de numA: ', cantIsDouble);
end.

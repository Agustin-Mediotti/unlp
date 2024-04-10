program ejercicio1;
var
  numA, numB: integer;
begin
  read(numA);
  read(numB);
  if (numA > numB) then
    writeln(numA, ' es el mayor.' );
  if (numB > numA) then
    writeln(numB, ' es el mayor.')
  else
    writeln('los numeros leidos son iguales.');
end.

{
  Algoritmo eficiente de busqueda del divisor comun mayor
  https://en.wikipedia.org/wiki/Euclidean_algorithm
  
  Agustin Mediotti
}

program euclidean_algorithm;

function euclid(m,k:integer):integer;
begin
  if k = 0 then
    euclid:= m
  else
    euclid(k, m MOD k);
end;

begin
  writeln(euclid(5,13))
end.

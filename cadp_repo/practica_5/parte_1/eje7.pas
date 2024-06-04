program ejercicio_7;

type
  str = string[50];
  cities_arr = array[2500] of str;

var
  cities: cities_arr;

begin
  writeln('inciso a: la estructura de datos pesa 123.5 kb o 127.500 bytes');
  writeln('inciso b_1: la variable punteros pesa 20k bytes o 19.5kb');
  writeln('inciso b_2: la cantidad de memoria reservada es de 127,500 bytes de memoria dinamica');
end.

{ 
  Realizar un programa que ocupe 50 KB de memoria en total. Para ello:
    a. El programa debe utilizar sólo memoria estática.
    b. El programa debe utilizar el 50% de memoria estática y el 50% de memoria dinámica.
    c. El programa debe minimizar tanto como sea posible el uso de la memoria estática (a lo sumo, 4 bytes).
}

program ejercicio_6_inciso_a;

type
  texto = string[49];

var
  str: texto;

begin
  str:= 'hello world';
  writeln(str)
end.

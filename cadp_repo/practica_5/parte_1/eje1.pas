{Indicar los valores que imprime el siguiente programa en Pascal. Justificar mediante prueba de
escritorio.}

program punteros;

type
  cadena = string[50];
  puntero_cadena = ^cadena;

var
  pc: puntero_cadena;

begin
  writeln(sizeof(pc), ' bytes'); // puntero de 8 bytes 
  new(pc);
  writeln(sizeof(pc), ' bytes'); // 8 bytes
  pc^:= 'un nuevo nombre';
  writeln(sizeof(pc), ' bytes'); // 8 bytes
  writeln(sizeof(pc^), ' bytes'); // string de 50 bytes + 1
  pc^:= 'otro nuevo nombre distinto al anterior';
  writeln(sizeof(pc^), ' bytes'); // 51 bytes
end.

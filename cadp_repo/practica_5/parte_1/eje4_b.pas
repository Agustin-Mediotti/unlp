{ inciso b }

program punteros;

type
  cadena = string[50];
  puntero_cadena = ^cadena;
  
var
  pc: puntero_cadena;
  
begin
  new(pc);
  pc^:= 'un nuevo nombre';
  writeln(sizeof(pc^), ' bytes');
  writeln(pc^);
  dispose(pc);
  pc^:= 'otro nuevo nombre'; // error al intentar acceder a una direccion de memoria que ya no existe.
end.


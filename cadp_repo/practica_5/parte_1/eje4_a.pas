{inciso a}

program punteros;

type
  cadena = string[50];
  puntero_cadena = ^cadena;
var
  pc: puntero_cadena;
    
begin
  pc^:= 'un nuevo texto'; // error en ejecucion, se accede a una posicion ilegal.
  new(pc);
  writeln(pc^);
end.

{ inciso d }

program punteros;
 
type
  cadena = string[50];
  puntero_cadena = ^cadena;
  
procedure cambiarTexto(pun: puntero_cadena);
begin
  new(pun);
  pun^:= 'Otro texto';
end;

var
  pc: puntero_cadena;

begin
  new(pc);
  pc^:= 'Un texto';
  writeln(pc^); -> Un texto
  cambiarTexto(pc);
  writeln(pc^); -> El programa imprime Otro texto pero la direccion de la primer variable queda perdida 
end.

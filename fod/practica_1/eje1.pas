program ejercicio_1;

type
  archivo_enteros = file of integer;

var
  archivo: archivo_enteros;
  path: string[12];
  num: integer;
begin
  write('Ingrese el nombre del archivo --> ');
  readln(path);

  assign(archivo, path);
  rewrite(archivo);

  write(archivo, 55);

  num:= 0;
  while(num <> 30000) do begin
    write('Ingrese un numero --> ');
    readln(num);
    if (num <> 30000) then
      write(archivo, num); 
  end;
  close(archivo);
end.

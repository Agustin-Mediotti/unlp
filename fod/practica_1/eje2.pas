program eje2;

type
  archivo_enteros = file of integer;

var
  archivo: archivo_enteros;
  num_actual, cant_menores_1500, total, suma: integer;
  path: string;

begin
  write('Ingrese el nombre del archivo --> ');
  readln(path);
  assign(archivo, path);
  reset(archivo);

  cant_menores_1500:=0;
  num_actual:=0;
  suma:=0;
  total:=0;

  while (not EOF(archivo)) do begin
    read(archivo, num_actual);
    if (num_actual < 1500) then
      cant_menores_1500:=cant_menores_1500+1;
    total:=total+1;
    suma:=suma+num_actual;
    write(num_actual, ' ');
  end;
  close(archivo);
  
  writeln('');
  writeln('El promedio es ', (suma/total):0:2);

  end.

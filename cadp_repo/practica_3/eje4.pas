program ejercicio_4;

type
  
  cliente = record
    cod: integer;
    cant_lineas: integer;
  end;

  linea = record
    num_telefono: integer;
    cant_min_consumidos: integer;
    cant_mb_consumidos: integer;
  end;

procedure leer_linea(l: linea);

begin
  writeln('Numero de telefono: ', l.num_telefono);
  writeln('Cantidad de minutos consumidos: ', l.cant_min_consumidos);
  writeln('Cantidad de MB consumidos: ', l.cant_mb_consumidos);
end;

procedure leer_cliente(var c: cliente);
  
  var
    l:linea;
    i, total_min, total_mb:integer;

begin

  total_min:=0;
  total_mb:=0;

  readln(c.cod);
  readln(c.cant_lineas);
  for i:= 1 to c.cant_lineas do begin
    readln(l.num_telefono);
    readln(l.cant_mb_consumidos);
    readln(l.cant_min_consumidos);
    total_mb:= total_mb+l.cant_mb_consumidos;
    total_min:=total_min+l.cant_min_consumidos;
    leer_linea(l);
  end;
  writeln('Minutos Consumidos: ', total_min, '. MB consumidos: ', total_mb , '. Total a facturar: $',((total_mb*1.35)+(total_min*3.40)):0:2);
end;

var
  c: cliente;
  i: integer;

begin
  for i:=1 to 9300 do begin
    leer_cliente(c);
  end;
end.

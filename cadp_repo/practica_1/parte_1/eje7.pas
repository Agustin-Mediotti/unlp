program ejercicio7;

var
  Codigo:integer;
  Precio, NuevoPrecio: real;
begin
  Codigo:=0;
  while(Codigo <> 32767) do begin
    read(Codigo);
    read(Precio);
    read(NuevoPrecio);
    if 100*(NuevoPrecio-Precio)/Precio <= 10.00 then
      writeln('El aumento de precio del producto ', Codigo, ' no supera el 10%')
    else
      writeln('El aumento de precio del producto ', Codigo, ' es superior al 10%');
  end;
end.

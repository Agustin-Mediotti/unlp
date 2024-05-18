program ejercicio_5;

type
  auto = record
    marca: string;
    modelo: string;
    precio: real;
  end;

var
  a: auto;
  mas_caro_precio, promedio: real;
  mas_caro_marca: string;
  mas_caro_modelo: string;
  marca_actual:string;
  cant_promedio: integer;

begin
  
  mas_caro_precio:=-9999;
  promedio:=0;
  mas_caro_marca:='';
  mas_caro_modelo:='';
  cant_promedio:=0;

  readln(a.marca);
  marca_actual:=a.marca;
  while a.marca <> 'ZZZ' do begin
    readln(a.modelo);
    readln(a.precio);
    if a.precio > mas_caro_precio then begin
      mas_caro_precio:= a.precio;
      mas_caro_marca:= a.marca;
      mas_caro_modelo:= a.modelo;
    end;
    if marca_actual <> a.marca then begin
      writeln('promedio de marca ', marca_actual, ' es de $', (promedio/cant_promedio):0:2);
      marca_actual:= a.marca;
    end
    else begin
      promedio:= promedio+a.precio;
      cant_promedio:= cant_promedio+1;
    end;
    readln(a.marca);
  end;
  writeln('Marca: ', mas_caro_marca,'. Modelo: ', mas_caro_modelo, '. Precio: $', mas_caro_precio:0:2);
end.

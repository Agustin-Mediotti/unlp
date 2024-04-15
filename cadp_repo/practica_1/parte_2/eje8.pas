program ejercicio_8_a;

var
  monto, i, ventas_por_dia, total_mes, dia_con_mayor_ventas, fecha_mayor_ventas: integer;
begin
  total_mes:=0;
  dia_con_mayor_ventas:=0;
  fecha_mayor_ventas:=0;
  for i:=1 to 5 do begin
    monto:=-1;
    ventas_por_dia:=0;
    while(monto <> 0) do begin
      readln(monto);
      if monto <> 0 then begin
        ventas_por_dia:=ventas_por_dia+1;
        total_mes:=total_mes+monto;
      end;
    end;
    writeln('Ventas del dia ', i, ': ', ventas_por_dia, '. Total acumulado: ', total_mes);
    if ventas_por_dia > dia_con_mayor_ventas then begin
      fecha_mayor_ventas:= i;
      dia_con_mayor_ventas:=ventas_por_dia;
    end;
  end;
  writeln('Dia con mas ventas: ', fecha_mayor_ventas);
end.

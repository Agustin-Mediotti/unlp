program ejercicio6;

var
  NumeroLejago, AlumnosTotal, AlumnosAprobados, AlumnosDestacados: integer;
  Promedio: real;
begin
  NumeroLejago:=0;
  AlumnosTotal:=0;
  AlumnosDestacados:=0;
  AlumnosAprobados:=0;
  while(NumeroLejago <> -1) do begin
    read(NumeroLejago);
    if NumeroLejago <> -1 then begin
      read(Promedio);
      AlumnosTotal:= AlumnosTotal+1;
      if (NumeroLejago < 2500) and (Promedio > 8.5) then
        begin
          AlumnosDestacados:= AlumnosDestacados+1;
        end;
      if Promedio > 6.5 then
        AlumnosAprobados:= AlumnosAprobados+1;
    end;
  end;
  writeln('Alumnos Totales: ', AlumnosTotal);
  writeln('Alumnos cuyo promedio supera 6.5: ', AlumnosAprobados);
  writeln('Porcentaje de alumnos destacados: ', (100*AlumnosDestacados)/AlumnosTotal:0:2,'%');
end.

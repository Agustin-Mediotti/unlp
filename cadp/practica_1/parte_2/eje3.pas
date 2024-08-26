program ejercicio_3;

const
  FINAL_NAME = 'Zidane Zinedine';
var
  name: string;
  nota: 1..10;
  cant_alumnos_aprobados, cant_alumnos_nota_7: integer;
begin
  nota:=1;
  cant_alumnos_aprobados:=0;
  cant_alumnos_nota_7:=0;
  name:='';
  while (name <> FINAL_NAME) do begin
    readln(name);
    readln(nota);
    if nota >= 8 then
      cant_alumnos_aprobados:= cant_alumnos_aprobados+1;
    if nota = 7 then
      cant_alumnos_nota_7:= cant_alumnos_nota_7+1;
  end;
  writeln('alumnos aprobados: ', cant_alumnos_aprobados);
  writeln('alumnos con nota 7: ', cant_alumnos_nota_7);
end.

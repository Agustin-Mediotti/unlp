program ejercicio_2;

var
  i, legajo, nota, cant_ingr, cant_ingr_rend, cant_rec, cant_rec_rend, cant_promedio_6, cant_aprobaron_todas, mas_nota_0_sub: integer;
  cant_notas_10, cant_nota_0, mas_nota_10_cod, cant_alumnos_nota_0, mas_nota_0_cant, mas_nota_0_cod, mas_nota_10_cant, mas_nota_10_sub: integer;
  condicion: 'I'..'R';
  promedio: real;
  aprobado: boolean;
begin
  cant_aprobaron_todas:=0;
  cant_promedio_6:=0;
  cant_alumnos_nota_0:=0;
  mas_nota_0_cant:=0;
  mas_nota_0_cod:=-1;
  mas_nota_10_cant:=0;
  mas_nota_10_sub:=-1;
  mas_nota_10_cod:=-1;
  mas_nota_0_sub:=-1;
  cant_ingr_rend:=0;
  cant_ingr:=0;
  cant_rec:=0;
  cant_rec_rend:=0;
  legajo:=0;
  while(legajo <> -1) do begin
    nota:=0;
    cant_notas_10:=0;
    promedio:=0;
    cant_nota_0:=0;
    aprobado:=true;
    writeln('Legajo: ');
    readln(legajo);
    if (legajo <> -1) then begin
      writeln('Condicion: ');
      readln(condicion);
      writeln('Notas: ');
      for i:=1 to 5 do begin
        readln(nota);
        if nota <> -1 then
          promedio:=promedio+nota
        else
          promedio:=promedio-1;
        if nota = 10 then
          cant_notas_10:=cant_notas_10+1;
        if nota = 0 then
          cant_nota_0:=cant_nota_0+1;
        if nota < 4 then
          aprobado:= false;
      end;
      if aprobado then
        cant_aprobaron_todas:=cant_aprobaron_todas+1;
      if cant_notas_10 > mas_nota_10_cant then begin
        mas_nota_10_sub:=mas_nota_10_cod;
        mas_nota_10_cant:=cant_notas_10;
        mas_nota_10_cod:=legajo;
      end;
      if cant_nota_0 > 0 then begin
        cant_alumnos_nota_0:=cant_alumnos_nota_0+1;
        if cant_nota_0 > mas_nota_0_cant then begin
          mas_nota_0_sub:=mas_nota_0_cod;
          mas_nota_0_cant:=cant_nota_0;
          mas_nota_0_cod:=legajo;
        end;
      end;
      if ((promedio/5) > 6.5) then
        cant_promedio_6:=cant_promedio_6+1;
      case condicion of
        'I' : begin
          cant_ingr:=cant_ingr+1;
          if aprobado then
            cant_ingr_rend:=cant_ingr_rend+1;
        end;
        'R' : begin
          cant_rec:= cant_rec+1;
          if aprobado then
            cant_rec_rend:=cant_rec_rend+1;
        end;
      end;
    end;
  end;
  writeln('Cantidad de alumnos INGRESANTES en condiciones de rendir parcial: ', cant_ingr_rend, ' de un total de ', cant_ingr, ': ', cant_ingr_rend/cant_ingr:0:2,'%');
  writeln('Cantidad de alumnos RECURSANTES en condiciones de rendir parcial: ', cant_rec_rend, ' de un total de ', cant_rec, ': ', cant_rec_rend/cant_rec:0:2,'%');
  writeln('Cantidad de alumnos que aprobaron todas las autoevaluaciones: ', cant_aprobaron_todas);
  writeln('Cantidad de alumnos cuya nota promedio fue mayor a 6.5 puntos: ', cant_promedio_6);
  writeln('Cantidad de alumnos que obtuvieron 0 puntos en al menos una evaluacion: ', cant_alumnos_nota_0);
  writeln('Lejagos de los dos alumnos con mayor cantidad de autoevaluaciones con nota 10: ', mas_nota_10_cod, ' y ', mas_nota_10_sub);
  writeln('Lejagos de los dos alumnos con mayor cantidad de autoevaluaciones con nota 0: ', mas_nota_0_cod, ' y ', mas_nota_0_sub);
end.

program ejercicio_9;

procedure procesar_apellidos(num_ins: integer; var apellido_menor_num_val: integer; var apellido_menor_num, apellido_menor_num2: string);
var
  apellido: string;
begin
  readln(apellido);
  if (num_ins < apellido_menor_num_val) then begin
    apellido_menor_num2:= apellido_menor_num;
    apellido_menor_num_val:= num_ins;
    apellido_menor_num:= apellido;
  end;
end;

procedure procesar_nombres(num_ins: integer; var nombre_mayor_val: integer; var nombre_mayor, nombre_mayor2: string);
var
  nombre: string;
begin
  readln(nombre);
  if (num_ins > nombre_mayor_val) then begin
    nombre_mayor2:= nombre_mayor;
    nombre_mayor_val:= num_ins;
    nombre_mayor:= nombre;
  end;
end;

var
  apellido_menor_num, apellido_menor_num2, nombre_mayor, nombre_mayor2: string;
  num_ins, cant_alumnos, cant_alumnos_num_par, apellido_menor_num_val, nombre_mayor_val: integer;

begin
  cant_alumnos:=0;
  cant_alumnos_num_par:=0;
  apellido_menor_num_val:=9999;
  nombre_mayor_val:=-9999;
  num_ins:=0;
  while (num_ins <> 1200) do begin
    readln(num_ins);
    procesar_apellidos (num_ins, apellido_menor_num_val, apellido_menor_num, apellido_menor_num2);
    procesar_nombres (num_ins, nombre_mayor_val, nombre_mayor, nombre_mayor2);
    if num_ins MOD 2 = 0 then
      cant_alumnos_num_par:= cant_alumnos_num_par+1;
    cant_alumnos:= cant_alumnos+1;
  end;
  writeln('apellido de los numeros de inscripcion mas chico: ', apellido_menor_num, ' y ', apellido_menor_num2);
  writeln('nombre de los dos numeros de inscripcion mas grandes: ', nombre_mayor, ' y ', nombre_mayor2);
  writeln('el porcentaje de numeros de insc. pares: ', (cant_alumnos_num_par*100)/cant_alumnos:0:2, '%');
end.

program ejercicio_3;

const UNESCO_PROP = 0.000042671;

type 
  establecimiento = record
    cue: integer;
    nombre: string;
    cant_docentes: integer;
    cant_alumnos: integer;
    localidad: string;
  end;

procedure leer_escuela(var escuela: establecimiento);
begin
  readln(escuela.cue);
  readln(escuela.nombre);
  readln(escuela.cant_docentes);
  readln(escuela.cant_alumnos);
  readln(escuela.localidad);
end;

function relacion_docente_alumno(escuela: establecimiento): real;
begin
  relacion_docente_alumno:= escuela.cant_alumnos / escuela.cant_docentes;
end;

var
  escuela: establecimiento;
  i, cant_lp_unesco, cue_mejor_relacion, cue_2do_mejor_relacion: integer;
  nombre_mejor_relacion, nombre_2do_mejor_relacion: string;
  rel_doc_alum, mejor_rel: real;

begin
  cant_lp_unesco:=0;
  mejor_rel:=0;
  cue_mejor_relacion:=0;
  cue_2do_mejor_relacion:=0;
  nombre_mejor_relacion:='';
  nombre_2do_mejor_relacion:='';

  for i:=1 to 2400 do begin
    leer_escuela(escuela);
    rel_doc_alum:= relacion_docente_alumno(escuela);

    if ((escuela.nombre = 'La Plata') and (rel_doc_alum >= UNESCO_PROP)) then
      cant_lp_unesco:=cant_lp_unesco+1;
    if (rel_doc_alum > mejor_rel) then begin
      cue_2do_mejor_relacion:= cue_mejor_relacion;
      nombre_2do_mejor_relacion:= nombre_mejor_relacion;
      cue_mejor_relacion:= escuela.cue;
      nombre_mejor_relacion:= escuela.nombre;
      mejor_rel:= rel_doc_alum;
    end;
  end;
  writeln('cantidad de colegios en La Plata con rel. de alumnos por docente superior a la sugerida por la UNESCO: ', cant_lp_unesco);
  writeln('CUE y nombre de las dos escuelas con mejor relacion entre docentes y alumnos: ');
  writeln('ESTABLECIMIENTO: ', nombre_mejor_relacion, ', CUE: ', cue_mejor_relacion);
  writeln('ESTABLECIMIENTO: ', nombre_2do_mejor_relacion, 'CUE: ', cue_2do_mejor_relacion);
end.

program ejercicio_12;

type
  t_zona = 1..3;

const
  PRECIO_TON = 320;

function calc_rend(ha: integer; t_zona: t_zona; precio_ton: real): real; { inciso a }
begin
  case t_zona of
    1: calc_rend:= (ha*6)*precio_ton;
    2: calc_rend:= (ha*2.6)*precio_ton;
    3: calc_rend:= (ha*1.4)*precio_ton;
  end;
end;

procedure proc_campo(var cantCampTresDeFebRendSup10k, cantCamposTotal: integer; sumaRend: real; var locMenorRend, locMayorRend: string); { inciso b }
var
  localidad: string;
  ha: integer;
  zona: t_zona;
  rend_est, locMenorRendVal, locMayorRendVal: real;

begin
  localidad:='';
  ha:=0;
  locMayorRendVal:=-9999;
  locMenorRendVal:=9999;

  while ((localidad <> 'Saladillo') and (ha = 900)) do begin
    readln(localidad);
    readln(ha);
    readln(zona);
    
    rend_est:= calc_rend(ha, zona, PRECIO_TON);

    if ((localidad = 'Tres de Febrero') and (rend_est > 10000)) then
      cantCampTresDeFebRendSup10k:= cantCampTresDeFebRendSup10k+1;

    if rend_est > locMayorRendVal then begin
      locMayorRend:= localidad;
      locMayorRendVal:= rend_est;
    end;

    if rend_est < locMenorRendVal then begin
      locMenorRend:= localidad;
      locMenorRendVal:= rend_est;
    end;

    sumaRend:= sumaRend+rend_est;
    cantCamposTotal:=cantCamposTotal+1;
  end;
end;

procedure generarInforme(cantCampTresDeFebRendSup10k, cantCamposTotal: integer; locMayorRend, locMenorRend: string; sumaRend: real);
begin
  writeln('cantidad de campos de la localidad Tres de Febrero con rendimiento estimado superior a U$S10k: ', cantCampTresDeFebRendSup10k);
  writeln('localidad del campo con mayor rendimiento economico esperado: ', locMayorRend);
  writeln('localidad del campo con menor rendimiento economico esperado: ', locMenorRend);
  writeln('rendimiento economico promedio: ', sumaRend/cantCamposTotal);
end;

var
  cantCampTresDeFebRendSup10k, cantCamposTotal: integer;
  sumaRend: real;
  locMenorRend, locMayorRend: string;

begin
  
  cantCampTresDeFebRendSup10k:=0;
  sumaRend:=0;
  cantCamposTotal:=0;

  proc_campo(cantCampTresDeFebRendSup10k, cantCamposTotal, sumaRend, locMenorRend, locMayorRend);
  generarInforme(cantCampTresDeFebRendSup10k, cantCamposTotal, locMayorRend, locMenorRend, sumaRend);
end.

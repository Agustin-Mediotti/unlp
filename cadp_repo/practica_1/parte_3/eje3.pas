program ejercicio_3;

const
  PI: real = 3.14;
var
  tipo_tanque: char;
  ancho, alto, largo, radio: real;
  vol, tanq_may_vol, tanq_may_vol_2, vol_prom_cil, vol_prom_rec: real;
  cant_altura_inf, cant_vol_inf_800, tanq_cil_vend, tanq_rec_vend: integer;
begin
  tipo_tanque:='C';
  tanq_may_vol:=-9999;
  tanq_may_vol_2:=-9999;
  vol_prom_cil:=0;
  vol_prom_rec:=0;
  cant_altura_inf:=0;
  cant_vol_inf_800:=0;
  tanq_cil_vend:=0;
  tanq_rec_vend:=0;
  while(tipo_tanque <> 'Z') do begin
    ancho:=0;
    alto:=0;
    largo:=0;
    radio:=0;
    vol:=0;
    if tipo_tanque <> 'Z' then begin
      writeln('Tipo de tanque: ');
      readln(tipo_tanque);
      case tipo_tanque of
        'C' : begin
          writeln('Alto: ');
          readln(alto);
          writeln('Radio: ');
          readln(radio);
          vol:= PI*(radio*2)*alto;
          tanq_cil_vend:=tanq_cil_vend+1;
          vol_prom_cil:=vol_prom_cil+vol;
        end;
        'R' : begin
          writeln('Ancho: ');
          readln(ancho);
          writeln('Largo: ');
          readln(largo);
          writeln('Alto: ');
          readln(alto);
          vol:= ancho*largo*alto;
          tanq_rec_vend:=tanq_rec_vend+1;
          vol_prom_rec:=vol_prom_rec+vol;
        end;
      end;
      if vol > tanq_may_vol then begin
        tanq_may_vol_2:=tanq_may_vol;
        tanq_may_vol:=vol;
      end;
      if alto < 1.40 then
        cant_altura_inf:=cant_altura_inf+1;
      if vol < 800 then
        cant_vol_inf_800:=cant_vol_inf_800+1;
    end;
  end;
  writeln('Volumen de los dos mayores tanques vendidos: ', tanq_may_vol:0:2, ', ', tanq_may_vol_2:0:2);
  writeln('Volumen promedio de todos los tanques cilindricos vendidos: ', vol_prom_cil/tanq_cil_vend:0:2);
  writeln('Volumen promedio de todos los tanques rectangulares vendidos: ', vol_prom_rec:0:2);
  writeln('Cantidad de tanques cuyo alto sea menor a 1.40 metros: ', cant_altura_inf);
  writeln('Cantidad de tanques cuyo volumen sea menor a 800 metros cubicos: ', cant_vol_inf_800);
end.

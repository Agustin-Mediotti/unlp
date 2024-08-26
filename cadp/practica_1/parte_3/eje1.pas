program ejercicio_1;

var
  cod_emp, cant_inv, cant_emp_inv_50k, emp_mayor_inv, i: integer;
  monto_inv, monto_total_inv, emp_mayor_inv_monto: real;
begin
  cod_emp:=0;
  cant_emp_inv_50k:=0;
  emp_mayor_inv:=0;
  emp_mayor_inv_monto:=-9999;
  while (cod_emp <> 100) do begin
    monto_total_inv:=0;
    writeln('Ingrese un codigo de empresa: ');
    readln(cod_emp);
    writeln('Ingrese el monto de cada inversion: ');
    readln(cant_inv);
    writeln('Ingrese el monto de cada inversion: ');
    for i:=1 to cant_inv do begin
      readln(monto_inv);
      monto_total_inv:= monto_total_inv+monto_inv;
    end;
    if monto_total_inv > emp_mayor_inv_monto then begin
      emp_mayor_inv:=cod_emp;
      emp_mayor_inv_monto:=monto_total_inv;
    end;
    if monto_total_inv > 50000 then
      cant_emp_inv_50k:=cant_emp_inv_50k+1;
    writeln('Resultado del analisis: Empresa ', cod_emp, ' Monto promedio: ', monto_total_inv/cant_inv:0:2);
  end;
  writeln('La empresa ', emp_mayor_inv, ' es la que mayor dinero posee invertido ($', emp_mayor_inv_monto:0:2,').');
  writeln('Hay ', cant_emp_inv_50k, ' empresas con inversiones por mas de $50000');
end.

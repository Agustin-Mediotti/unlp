program ejercicio_4_b;

var
  num, n_min, n_min2: integer;
begin
  n_min:=9999;
  n_min2:=9999;
  num:=-1;
  while(num <> 0) do begin
    if num <> 0 then begin
      readln(num);
      if (num < n_min) and (num <> 0) then begin
        n_min2:= n_min;
        n_min:=num;
      end;
    end;
  end;
  writeln('numeros minimos: ', n_min, ' ', n_min2);
end.

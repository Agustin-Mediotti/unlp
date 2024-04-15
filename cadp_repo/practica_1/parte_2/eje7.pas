program ejercicio_7;
var
  name, first, second, last, at_last: string;
  time, i, first_t, second_t, last_t, at_last_t: integer;
begin
  first_t:=9999;
  second_t:=9999;
  last_t:=-9999;
  at_last_t:=-9999;
  first:='';
  last:=first;
  at_last:=first;
  second:=first;
  for i:=1 to 100 do begin
    readln(name);
    readln(time);
    if time > last_t then begin
      at_last_t:= last_t;
      last_t:= time;
      at_last:= last;
      last:=name;
    end;
    if time < first_t then begin
      second_t:= first_t;
      first_t:= time;
      second:=first;
      first:=name;
    end;
  end;
  writeln('1st puesto: ', first, ', 2do puesto: ', second, '. Anteultimo puesto: ', at_last, ', Ultimo puesto: ', last);
end.

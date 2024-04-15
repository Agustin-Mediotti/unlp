program ejercicio_1;

var
  total, num, plus_5, i: integer;
begin
  total:=0;
  plus_5:=0;
  for i:=1 to 10 do begin
    read(num);
    total:= total+num;
    if num > 5 then
      plus_5:= plus_5+1;
  end;
  writeln('total: ',total);
  writeln('mayores a 5: ', plus_5);
end.

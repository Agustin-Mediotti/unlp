program ejercicio_5;

var
  num, max, min, sum: integer;
begin
  num:=0;
  max:=-9999;
  min:=9999;
  sum:=0;
  while (num <> 100) do begin
    readln(num);
    if num > max then
      max:=num;
    if num < min then
      min:=num;
    sum:=sum+num;
  end;
  writeln('max: ', max, ' min: ', min, ' sum: ', sum);
end.

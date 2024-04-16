program ejercicio9;

type
  op = '+'..'-';
var
  operation: op;
  num, total: integer;
begin
  num:=-1;
  total:=0;
  read(operation);
  if (operation <> '+') and (operation <> '-') then
    writeln('Err: operation not found!')
  else begin
    while(num <> 0) do begin
      read(num);
      if operation = '+' then
        total:= total+num
      else
        total:= num-total;
    end;
    writeln('Total: ', total);
  end;
end.

program ejercicio_adicional_2;

function calc_producto(x,y,sum:integer): integer;
begin
  if sum = 0 then
    sum:= x;
  if x < y then begin
    sum:= sum * (x+1);
    calc_producto(x+1,y,sum);
  end
  else begin
    calc_producto:=sum;
    writeln(sum); { ?? sin eso no imprime }
  end;
end;

function calc_sum(x,y,sum: integer): integer;
begin
  if sum = 0 then
    sum:= x;
  if x < y then begin
    sum:= sum + (x+1);
    calc_sum(x+1, y, sum);
  end
  else begin
   calc_sum:= sum;
    writeln(sum); 
 end;
end;

var
  i,x,y,sum:integer;

begin
  for i:=1 to 10 do begin
    sum:=0;
    readln(x);
    readln(y);
    writeln('producto de los numeros (', x, ':', y, ') = ', calc_producto(x,y,sum));
    writeln('suma de los numeros (', x, ':', y, ') = ', calc_sum(x,y,sum));
  end;
end.

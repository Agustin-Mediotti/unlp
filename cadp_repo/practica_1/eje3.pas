program ejercicio3;

var
  a, b, c: integer;
begin
  read(a);
  read(b);
  read(c);
  if (a > b) and (a > c) then begin
    write(a);
    if b > c then begin
      write(b);
      write(c);
    end
    else begin
      write(c);
      write(b);
    end;
  end;
  if (b > a) and (b > c) then begin
    write(b);
    if a > c then begin
      write(a);
      write(c);
    end
    else begin
      write(c);
      write(b);
    end;
  end;
  if (c > b) and (c > a) then begin
    write(c);
    if b > a then begin
      write(b);
      write(a);
    end
    else begin
      write(a);
      write(b);
    end;
  end;
end.

program eje6;

procedure read_until_negative;
  var
    num, bigger: integer;
  begin
    num:=0;
    bigger:=-9999;
    while num >= 0 do begin
      read(num);
      if num > bigger then
        bigger:=num;
    end;
    writeln('el numero mas alto fue: ', bigger);
  end;
begin
  read_until_negative;
end.

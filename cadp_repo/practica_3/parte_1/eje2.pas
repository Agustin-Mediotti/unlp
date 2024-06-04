program ejeercicio_2;

type
  date_c = record { inciso a }
    year: integer;
    month: integer;
    day: integer;
  end;

procedure read_date(var new_date: date_c); begin { inciso b }
  readln(new_date.year);
  if new_date.year = 2019 then begin
    readln(new_date.month);
    readln(new_date.day);
  end;
end;

var { inciso c }
  first_days, summer, year: integer;
  new_date: date_c;

begin

  first_days:=0;
  summer:=0;
  year:= 2019;

  while(year = 2019) do begin
    read_date(new_date);
    year:=new_date.year;
    if ((new_date.month >= 1) and (new_date.month <= 3)) then
      summer:= summer+1;
    if new_date.day <= 10 then
      first_days:= first_days+1;
  end;

  writeln('casamientos en verano: ', summer);
  writeln('casamientos en los primeros 10 dias: ', first_days);
end.


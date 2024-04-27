program ejercicio_11;

procedure parteA (var c: char; var cantPartA: integer; var isPartA: boolean);
begin
  readln(c);
  while c <> '%' do begin
    if c = '$' then
      isPartA:= true;
    cantPartA:= cantPartA+1;
    readln(c);
  end;
end;

procedure parteB (var c: char; cantPartA: integer; var isPartB: boolean);
var
  arrobas, char_count: integer;
begin
  arrobas:=0;
  char_count:=0;
  readln(c);
  while c <> '*' do begin
    if c = '@' then
      arrobas:= arrobas+1;
    char_count:= char_count+1;
    readln(c);
  end;
  if (arrobas < 3) or (char_count <> cantPartA) then
    isPartB:=true;
end;

var
  c: char;
  cantPartA: integer;
  isPartA, isPartB: boolean;

begin
  parteA(c, cantPartA, isPartA);
  parteB(c, cantPartA, isPartB);
  if isPartA then
    writeln('parte A no se cumplio.');
  if isPartB then
    writeln('parte B no se cumplio.');
end.

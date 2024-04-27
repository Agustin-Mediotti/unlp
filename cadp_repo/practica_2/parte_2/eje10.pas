{Realizar un programa modularizado que lea una secuencia de caracteres y verifique si cumple con el patrón
A$B#, donde:
- A es una secuencia de sólo letras vocales
- B es una secuencia de sólo caracteres alfabéticos sin letras vocales
- los caracteres $ y # seguro existen
Nota: en caso de no cumplir, informar que parte del patrón no se cumplió.}

program ejercicio_10;

function esVocal (letra: char): boolean;
begin
	esVocal:= (letra = 'A') OR (letra = 'a') OR (letra ='e') OR (letra= 'E') OR (letra = 'i') OR (letra = 'I') OR (letra = 'u') OR (letra = 'U') OR (letra = 'O') or (letra = 'o');
end;

var
  letra: char;
  ok, partA, partB: boolean;

begin
  partA:= false;
  partB:= false;

  readln(letra);
  ok:= esVocal(letra);
  while (letra <> '$') do begin
    if ok then
      partA:= true;
    readln(letra);
    ok:=esVocal(letra);
  end;
  readln(letra);
  ok:= esVocal(letra);
  while (letra <> '#') do begin
    if not ok then
      partB:= true;
    readln(letra);
    ok:= esVocal(letra);
  end;
  if partA then
    writeln('la secuencia A no se cumplio.');
  if partB then
    writeln('la secuencia B no se cumplio');
end.

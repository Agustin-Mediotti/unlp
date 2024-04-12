program ejercicio8;

type
  letter = 'a'..'z';
var
  charA, charB, charC: letter;
begin
  read(charA);
  read(charB);
  read(charC);
  if (((charA = 'a') or (charA = 'e') or (charA = 'i') or (charA = 'o') or (charA = 'u')) and ((charB = 'a') or (charB = 'e') or (charB = 'i') or (charB = 'o') or (charB = 'u')) and ((charC = 'a') or (charC = 'e') or (charC = 'i') or (charC = 'o') or (charC = 'u'))) then
    writeln('Los tres valores son vocales')
  else
    writeln('Al menos un caracter no es vocal');
end.

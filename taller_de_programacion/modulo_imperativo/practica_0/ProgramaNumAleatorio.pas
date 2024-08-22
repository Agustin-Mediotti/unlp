program ProgramaNumAleatorio;

function rnd(A, B: integer):integer;
var ale: integer;
begin
  while (ale < A) do
    ale := random (B); {devuelve un valor aleatorio en el intervalo A..B}
  rnd:= ale;
end;

var n,A,B,F:integer;
begin
     randomize; {Elige una semilla distinta cada vez que se ejecuta el programa.}
    
    readln(A);
    readln(B);
    readln(F);
    
    while(n <> F) do begin  
      n:= rnd(A,B);
      if n <> F then
        writeln(n);
    end;

	 writeln ('Presione cualquier tecla para finalizar');
     readln;
end.

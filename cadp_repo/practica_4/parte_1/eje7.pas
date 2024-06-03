program ejercicio_7;

type
  vec = array [0..9] of integer;

procedure inicializar_vec(var v:vec);
var i:integer;
begin
  for i:=0 to 9 do begin
    v[i]:=0;
  end;
end;

function mas_contado(v:vec):integer;
var i,max,nmax:integer;
begin
  nmax:=0;
  for i:=0 to 9 do begin
    if v[i] > nmax then begin
      nmax:= v[i];
      max:= i;
    end;
  end;
  mas_contado:= max;
end;

procedure leer_ocurrencias(v:vec);
var i:integer;
begin
  for i:= 0 to 9 do begin
    if v[i] > 0 then
      writeln('Numero ', i,': ', v[i], ' veces');
  end;
end;

procedure sin_ocurrencias(v:vec);
var i:integer;
begin
  for i:=0 to 9 do begin
    if v[i] = 0 then
      writeln(i);
  end;
end;

procedure procesar_numeros(var v:vec);
var num:integer;
begin
  readln(num);
  while(num<>-1) do begin
    v[num]:= v[num]+1;
    readln(num);
  end;
end;

var
  v:vec;

begin
  inicializar_vec(v);
  procesar_numeros(v);
  leer_ocurrencias(v);
  writeln('el mas contado leido fue el ', mas_contado(v));
  writeln('los digitos sin ocurrencias son '); 
  sin_ocurrencias(v);
end.

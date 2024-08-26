{ 
  a. Máximo: recibe la lista como parámetro y retorna el elemento de valor máximo.
  b. Mínimo: recibe la lista como parámetro y retorna el elemento de valor mínimo.
  c. Múltiplos: recibe como parámetros la lista L y un valor entero A, y retorna la cantidad de
  elementos de la lista que son múltiplos de A.

}

program JugamosConListas;

type
  lista = ^nodo;
  nodo = record
    num : integer;
    sig : lista;
  end;

procedure armarNodo(var L: lista; v: integer);
var
  aux : lista;
begin
  new(aux);
  aux^.num := v;
  aux^.sig := L;
  L := aux;
end;

procedure imprimirLista(L: lista);
begin
  while (L <> nil) do begin
    writeln(L^.num);
    L:=L^.sig;
  end;
end;

function maximo(L:lista):lista;
var
  max:integer; elem:lista;
begin
  max:=-9999;
  while(L <> nil) do begin
    if L^.num > max then begin
      max:= L^.num;
      elem:= L;
    end;
    L:=L^.sig;
  end;
  maximo:=elem;
end;

function minimo(L:lista):lista;
var
  min:integer; elem:lista;
begin
  min:=9999;
  while(L <> nil) do begin
    if L^.num < min then begin
      min:= L^.num;
      elem:= L;
    end;
    L:=L^.sig;
  end;
  minimo:= elem;
end;

function multiplos(L:lista; A:integer):integer;
var aux:integer;
begin
  aux:=0;
  while(L <> nil) do begin
    if (A >= L^.num) and (A MOD L^.num = 0) then
      aux:=aux+1;
    L:=L^.sig;
  end;
  multiplos:=aux;
end;

procedure sumarElemento(var L: lista; v: integer);
var
  aux: lista;
begin
  aux:=L;
  while (L <> nil) do begin
    L^.num:= L^.num+v;
    L:=L^.sig;
  end;
  L:=aux;
end;

var
  pri : lista;
  valor : integer;

begin
  pri := nil;
  writeln('Ingrese un numero');
  read(valor);
  while (valor <> 0) do begin
    armarNodo(pri, valor);
    writeln('Ingrese un numero');
    read(valor);
  end;
  imprimirLista(pri);
  sumarElemento(pri, 10);
end.

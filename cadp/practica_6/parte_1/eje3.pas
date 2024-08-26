program JugamosConListas;

type
  lista = ^nodo;
  nodo = record
    num : integer;
    sig : lista;
  end;

procedure armarNodo(var L, Ult: lista; v: integer); // inciso a
var
  aux : lista;
begin
  new(aux);
  aux^.num := v;
  aux^.sig := nil;
  if L = nil then
    L:= aux
  else
    Ult^.sig = aux;
  Ult:= aux;
end;

procedure imprimirLista(L: lista);
begin
  while (L <> nil) do begin
    writeln(L^.num);
    L:=L^.sig;
  end;
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
  pri, ult : lista;
  valor : integer;

begin
  pri := nil;
  ult := nil; // inciso b
  writeln('Ingrese un numero');
  read(valor);
  while (valor <> 0) do begin
    armarNodo(pri, ult, valor);
    writeln('Ingrese un numero');
    read(valor);
  end;
  imprimirLista(pri);
  sumarElemento(pri, 10);
end.

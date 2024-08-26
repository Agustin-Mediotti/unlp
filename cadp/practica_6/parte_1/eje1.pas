{ 
  a) el programa lee numeros y los agrega a una lista hasta que se ingresa el numero 0 e imprime
  b) la lista quedaria: 48 13 21 10 0
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

procedure imprimirLista(L: lista); // inciso c
begin
  while (L <> nil) do begin
    writeln(L^.num);
    L:=L^.sig;
  end;
end;

procedure sumarElemento(var L: lista; v: integer); // inciso d
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

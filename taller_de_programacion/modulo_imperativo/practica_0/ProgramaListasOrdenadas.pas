program ListasOrdenadas;

const target = 120;

type
  lista = ^nodo;
  nodo = record
    dato: integer;
    sig: lista;
  end;

function rnd(A,B:integer):integer;
var num:integer;
begin
  num:=0;
  while num < A do
    num:= random (B);
  rnd:=num;
end;

procedure InsertarOrdenado(var L:lista; num:integer);
var nue,act,ant:lista;
begin
  new(nue);
  act:=L;
  ant:=L;
  nue^.dato:=num;
  while (act <> nil) and (act^.dato < num) do begin
    ant:=act;
    act:=act^.sig;
  end;
  if act = ant then
    L:=nue
  else
    ant^.sig:=nue;
  nue^.sig:=act;
end;

procedure CargarListaOrdenada(var L:lista);
var num:integer;
begin
  num:= rnd(100,150);
  while num <> target do begin
    InsertarOrdenado(L,num);
    num:= rnd(100, 150);
  end;
end;

function BuscarElementoOrdenado(L:lista; e:integer):boolean;
var existe:boolean;
begin
  existe:=false;
  while (L <> nil) and (not existe) do begin
    if L^.dato = e then
      existe:=true;
    L:=L^.sig;
  end;
  BuscarElementoOrdenado:=existe;
end;

procedure ImprimirLista(L:lista);
begin
  while L <> nil do begin
    writeln(L^.dato);
    L:=L^.sig;
  end;
end;

var L:lista; e:integer;
begin
  L:=nil;
  CargarListaOrdenada(L);
  ImprimirLista(L);
  readln(e);
  if BuscarElementoOrdenado(L, e) then
    writeln('Existe en la lista.')
  else
    writeln('No existe en la lista.');
end.

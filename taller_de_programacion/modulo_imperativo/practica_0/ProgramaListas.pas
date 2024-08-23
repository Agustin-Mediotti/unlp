program ListasAleatorias;

const target = 120;

type
  lista = ^nodo;
  nodo = record
    dato: integer;
    sig:lista;
  end;

function rnd(A,B:integer):integer;
var num:integer;
begin
  num:=0;
  while num < A do
    num:=random (B);
  rnd:= num;
end;

procedure AgregarAdelante(var L:lista; num:integer);
var nue:lista;
begin
  new(nue);
  nue^.dato:= num;
  nue^.sig:= L;
  L:= nue;
end;

procedure CargarLista(var L:lista);
var aux: integer;
begin
  aux:=rnd(100,150);
  while aux <> 120 do begin
    AgregarAdelante(L,aux);
    aux:=rnd(100,150);
  end;
end;

procedure ImprimirLista(L:lista);
begin
  while L <> nil do begin
    writeln(L^.dato);
    L:=L^.sig;
  end;
end;

var
  L:lista;
begin
  randomize;

  L:=nil;
  CargarLista(L);
  ImprimirLista(L); 
end.

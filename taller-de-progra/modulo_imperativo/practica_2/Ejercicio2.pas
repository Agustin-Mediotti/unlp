program Ejercicio_2;

const FIN = 100;

type
  rango = 100..200;
  lista = ^nodo;
  nodo = record
    dato: integer;
    sig: lista;
  end;

procedure CargarListaRecursivo(var L, Ult: lista);

  function NumeroRandom():integer;
  var n: integer;
  begin
    n:= random(High(rango));
    while n < Low(rango) do
      n:= random(High(rango));
    NumeroRandom:= n;
  end;

  procedure AgregarAtras(var L, Ult: lista; e: integer);
  var nue: lista;
  begin
    new(nue);
    nue^.dato:= e;
    nue^.sig:= nil;
    if L = nil then
      L:= nue
    else
      Ult^.sig:= nue;
    Ult:= nue;
  end;

var
  nue: integer;
begin
  nue:= NumeroRandom();
  if nue <> 100 then begin
    AgregarAtras(L,Ult,nue);
    CargarListaRecursivo(L, Ult);
  end;
end;

procedure ImprimirLista(L,Ult: lista);
begin
  if L <> Ult then begin
    writeln(L^.dato);
    ImprimirLista(L^.sig, Ult);
  end;
end;

procedure ImprimirListaInverso(L, Ult: lista);
begin
  if L <> Ult then begin
    ImprimirListaInverso(L^.sig, Ult);
    writeln(L^.dato);
  end;
end;

procedure ImprimirMinimo(L, Ult: lista);

  function ObtenerMenor(x,y:integer):integer;
   begin
     if x < y then ObtenerMenor:=x else ObtenerMenor:=y;
   end;

  function BuscarMinimoRecursivo(L, Ult: lista): integer;
  begin
    if L = Ult then
      BuscarMinimoRecursivo:= 9999
    else
      BuscarMinimoRecursivo:= ObtenerMenor(L^.dato, BuscarMinimoRecursivo(L^.sig, Ult));
  end;

begin
  writeln('Minimo: ', BuscarMinimoRecursivo(L, Ult));
end;

function ExisteEnLaLista(L,Ult:lista; num:integer):boolean;
begin
  if L = Ult then begin
    ExisteEnLaLista:= L^.dato = num;
  end else if L^.dato = num then
    ExisteEnLaLista:= true
  else
    ExisteEnLaLista(L^.sig, Ult, num);
end;

var
  L, Ult: lista;
begin
  randomize;
  L:= nil;
  Ult:= nil;

  CargarListaRecursivo(L, Ult);           {inciso a}
  ImprimirLista(L, Ult);                  {inciso b}
  ImprimirListaInverso(L, Ult);           {inciso c}
  ImprimirMinimo(L, Ult);                 {inciso d}
  writeln(ExisteEnLaLista(L, Ult, 105));  {inciso e}
end.

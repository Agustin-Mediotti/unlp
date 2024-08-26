/*
  Procedimiento Agregar Atras
*/

program agregar_atras;

type
  jugador = record
    nombre: string;
    altura: real;
    dni: integer;
  end;

  lista = ^nodo;

  nodo = record
    dato: jugador;
    sig: lista;
  end;

procedure LeerJugador(var J: jugador);
begin
  readln(J.dni);
  readln(J.nombre);
  readln(J.altura);
end;

procedure CargarLista(var L: lista);
var ULT: lista; J: jugador;
begin
  LeerJugador(J);
  while(J.dni <> 0) do begin
    AgregarAtras(J, L, ULT);
    LeerJugador(J);
  end;
end;

procedure AgregarAtras(J: jugador; var L, ULT: lista);
var nue: lista;
begin
  new(nue);
  nue^.dato:= J;
  nue^.sig:= NIL;
  
  if (L = NIL) then
    L:= nue
  else
    ULT^.sig:= nue;
  ULT:= nue;
end;

var
  L: lista;
begin
  L:=NIL;
  CargarLista(L);
end.

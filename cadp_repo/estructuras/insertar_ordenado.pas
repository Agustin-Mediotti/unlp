/*
  Procedimiento Insertar Ordenado
*/


program insertar_ordenado;

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
  readln(J.nombre);
  readln(J.altura);
  readln(J.dni);
end;

procedure CargarLista(var L: lista);
J: jugador;
begin
  LeerJugador(J);
  while (J.dni <> 0) do begin
    InsertarOrdenado(J, L);
    LeerJugador(J);
  end;
end;

procedure InsertarOrdenado(J: jugador; var L: lista);
var
  nue, act, ant: lista;
begin
  new(nue);
  nue^.dato:= J;
  act:=L;
  ant:=L;
  
  while(act <> NIL and act^.dato.altura < J.altura) do begin
    ant:=act;
    act:=act^.sig;
  end;
  
  if (act = ant) then
    L:=nue;
  else
    ant^.sig:=nue;
  nue.sig:=act;
end;

var
  L:lista;
begin
  L:=nil;
  CargarLista(L);
end.

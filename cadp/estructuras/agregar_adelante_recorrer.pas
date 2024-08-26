/*
  Agregar Adelante y Recorrer
*/

program agregar_adelante_y_recorrer;

type
  jugador = record
    dni: integer;
    nombre: string[30];
    altura: integer;
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

procedure AgregarAdelante(var L: lista; J: jugador);
var nue: lista;
begin
  new(nue);
  nue.dato:=J;
  nue.sig:=L;
  L:=nue;
end;

procedure CargaLista(var L: lista);
var J: jugador;
begin
  LeerJugador(J);
  while(J.dni <> 0) do begin
    AgregarAdelante(L, J);
    LeerJugador(J);
  end;
end;

procedure RecorrerLista(L: lista): lista;
begin
  while L <> NIL do begin
    if L.dato.dni = 666 then
      RecorrerLista:= L;
  end;
  RecorrerLista:= NIL;
end;

var
  L: lista;
begin
  L:=NIL;
  CargarLista(L);
end.

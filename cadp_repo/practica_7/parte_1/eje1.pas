program ejercicio_1;

type
  cod_genero = 1..5;
  lista = ^nodo;
  persona = record
    dni: integer;
    apellido: string;
    nombre: string;
    edad: integer;
    cod: cod_genero;
  end;

  nodo = record
    dato: persona;
    sig: lista;
  end;

procedure agregarAdelante(var L: lista; per: persona);
var nue: lista;
begin
  new(nue);
  nue^.sig:=L;
  nue^.dato:=persona;
  L:=nue;
end;

procedure cargarLista(var L:lista);
var per: persona;
begin

  leerPersona(per);
  while aux.dni <> 33555444 do begin
    agregarAdelante(L, per);
    leerPersona(per);
  end;
end;

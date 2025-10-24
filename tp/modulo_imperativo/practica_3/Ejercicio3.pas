program Ejercicio3.pas;

const
  FIN_LECTURA = 0;
  MAX_LEJAGO = 100;

type
  rng_nota = 1..10;
  rng_cod = 1..19;
  rng_dd = 1..31;
  rng_mm = 1..12; 
  rng_yy = 2020..2024;

  date = record
    dd: integer;
    mm: integer;
    yy: integer;
  end;

  examen_final = record
    legajo: integer;
    cod_materia: integer;
    fecha: date;
    nota: rng_nota;
  end;

  lista = ^nodo;
  arbol = ^nodo_arbol;

  nodo = record
    dato: examen_final;
    sig: lista;
  end;

  nodo_arbol = record
    dato: lista;
    HI: arbol;
    HD: arbol;
  end;

function fechaRandom():date;
var d: date;
begin
  d.dd:=random(High(rng_yy)); d.mm:=random(High(rng_mm)); d.dd:=random(High(rng_dd));
  
  while d.dd < Low(rng_dd) do
    d.dd:=random(High(rng_dd));

  while d.mm < Low(rng_mm) do
    d.mm:=random(High(rng_mm));

  while d.yy < Low(rng_yy) do
    d.yy:=random(High(rng_yy));

  fechaRandom:=d;
end;

procedure CargarDatos(var a: arbol);

  procedure AgregarAlArbol(var a: arbol; ex: examen_final);
  begin
    if a = nil then begin
      new(a);
      a^.dato:=ex;
      a^.HI:= nil;
      a^.HD:= nil;
    end else if ex.legajo <= a^.dato.legajo
  end;

  procedure LeerFinal(var ex: examen_final);
  begin
    ex.legajo:= random(MAX_LEJAGO);
    if ex.legajo <> FIN_LECTURA then begin
      ex.cod_materia:= random(High(rng_cod));
      ex.fecha:= fechaRandom();
      ex.nota:= random(High(rng_nota));
    end;
  end;

var e: examen_final;
begin
  LeerFinal(e);
  while(e.legajo <> FIN_LECTURA) then begin
    AgregarAlArbol(a, e);
    LeerFinal(e);
  end;
end;

var a: arbol;
begin
  a:=nil;
  CargarDatos(a);
end.

program ejercicio_5;

type
  lista = ^nodo;
  producto = record
    cod: integer;
    descripcion: string;
    stock_act: integer;
    stock_min: integer;
    precio:real;
  end;

  nodo = record
    dato: producto;
    sig: lista;
  end;

procedure leerProducto(var prod: producto);
begin
    readln(prod.cod);
    readln(prod.descripcion);
    readln(prod.stock_act);
    readln(prod.stock_min);
    readln(prod.precio);
end;

procedure agregarNodo(var L: lista; prod: producto); // LIFO
var nue: lista;
begin
  new(nue);
  nue^.dato:= prod;
  nue^.sig:= L;
  L := nue;
end;

procedure cargarLista(L:lista);
var prod: producto;
begin
  leerProducto(prod);
  while(prod.cod <> -1) do begin
    agregarNodo(L, prod);
    leerProducto(prod);
  end;
end;

function tresDigPares(cod:integer):boolean;
var aux, dig, cant: integer;
begin
  cant:=0;
  aux:= cod;
  while(aux <> 0) and (cant < 3) do begin
    dig:= aux MOD 10;
    if dig MOD 2 = 0 then
      cant:= cant+1;
    aux:= aux DIV 10;
  end;
  tresDigPares:= cant >=3;
end;

procedure procesarLista(L: lista);
var cant_low_stock, cod_eco, cod_eco2, total: integer; precio_eco: real;
begin
  total:=0;
  cant_low_stock:=0;
  precio_eco:=-9999;
  cod_eco:=0;
  cod_eco2:=0;
  while(L <> nil) do begin
    if L^.dato.stock_act < L^.dato.stock_min then
      cant_low_stock:= cant_low_stock+1;
    if tresDigPares(L^.dato.cod) then
      writeln('descripcion de codigo de producto con al menos 3 numeros pares: ', L^.dato.descripcion);
    total:=total+1;
    if L^.dato.precio < precio_eco then begin
      cod_eco2:= cod_eco;
      precio_eco:= L^.dato.precio;
      cod_eco:= L^.dato.cod;
    end;
  end;
  writeln('cantidad de productos con stock actual por debajo del stock minimo: ', (cant_low_stock / total):0:2, '%' );
  writeln('codigo de los dos productos mas economicos: ', cod_eco, ' y ', cod_eco2);
end;

var
  pri: lista;

begin
  pri:=nil;
  cargarLista(pri);
  procesarLista(pri);
end.

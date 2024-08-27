program Ejercicio_4;

const
  RUBROS = 8;
  PRODUCTOS = 30;
  COD_FIN = 0;

type
  rng_cod_rubro = 1..RUBROS;
  rng_ele_rubro_3 = 1..PRODUCTOS;
  producto = record
    cod_prod: integer;
    cod_rub:  rng_cod_rubro;
    precio:   real;
  end;
  lista = ^nodo;
  nodo = record
    dato: producto;
    sig:  lista;
  end;
  vec_ls_prod = array[rng_cod_rubro] of lista;
  vec_rubro_3 = array[rng_ele_rubro_3] of producto;

procedure InicializarListas(var vec:vec_ls_prod);
var
  i: integer;
begin
  for i:=1 to RUBROS do begin
    vec[i]:=nil;
  end;
end;

procedure LeerProducto(var prod:producto);
begin
  readln(prod.cod_prod);
  readln(prod.cod_rub);
  readln(prod.precio);
end;

procedure InsertarOrdenado(var L:lista; prod:producto);
var
  nue, act, ant:lista;
begin
  new(nue);
  nue^.dato:=prod;
  act:= L;
  ant:= L;
  while act <> ant do begin
    ant:= act;
    act:= ant^.sig;
  end;
  if act = ant then
    L:= nue
  else
    ant^.sig:=nue;
  nue^.sig:=act;
end;

procedure CargarProductos(var vec:vec_ls_prod);
var
  prod:producto;
begin
  InicializarListas(vec);
  LeerProducto(prod);
  while prod.precio <> COD_FIN do begin
    InsertarOrdenado(vec[prod.cod_rub], prod);
    LeerProducto(prod);
  end;
end;

function getDimL(L:lista):integer;
var
  cant: integer;
begin
  cant:=0;
  while ((L <> nil) and (cant <= 30 )) do begin
    cant:= cant +1;
    L:=L^.sig;
  end;
  getDimL:= cant;
end;

procedure PrimerosProductosRubro3(vec_productos:vec_ls_prod; var vec_rubro3:vec_rubro_3);
var
  i, dimL: integer;
  L: lista;
begin
  L:= vec_productos[3];
  dimL:= getDimL(L);
  if dimL > PRODUCTOS then begin
    for i:=1 to dimL do begin
      vec_rubro3[i]:= L^.dato;
      L:=L^.sig;
    end;
  end;
end;

procedure OrdenarSeleccion(var vec: vec_rubro_3);
begin
  // TODO: Completar.
end;

procedure ImprimirPreciosOrdenados(vec: vec_rubro_3);
begin
  // TODO: Completar.
end;

procedure CalcularPromedio(vec: vec_rubro_3);
begin
  // TODO: Completar.
end;

procedure ImprimirCodigosPorRubro(vec:vec_ls_prod);   // TODO: Hacer procedimineto recursivo.
var
  i: integer;
begin
  for i:=1 to RUBROS do begin
    while vec[i] <> nil do begin
      writeln('RUBRO: ', i, ', ', ' CODIGO: ', vec[i]^.dato.cod_prod);
    end;
  end;
end;

var
  vec_productos: vec_ls_prod;
  vec_prod_rubro3: vec_rubro_3;
begin
  CargarProductos(vec_productos);                           {inciso a}
  ImprimirCodigosPorRubro(vec_productos);                   {inciso b}
  PrimerosProductosRubro3(vec_productos, vec_prod_rubro3);  {inciso c}
  OrdenarSeleccion(vec_prod_rubro3);                        {inciso d}
  ImprimirPreciosOrdenados(vec_prod_rubro3);                {inciso e}
  CalcularPromedio(vec_prod_rubro3);                        {inciso f}
end.

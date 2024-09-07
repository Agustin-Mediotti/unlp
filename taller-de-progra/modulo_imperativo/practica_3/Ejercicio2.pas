program Ejercicio_2;

const
  COD_PROD = 30;
  MAX_UNIDADES_VENDIDAS = 50;

type
  rango_yyyy = 1950..2024;
  rango_mm = 1..12;
  rango_dd = 1..31;
 
  date = record
    yyyy: integer;
    mm:   integer;
    dd:   integer; 
  end;

  venta = record
    cod_prod: integer;
    fecha: date;
    cant_unidades_vendidas: integer;
  end;

  venta_record = record
    cod_prod: integer;
    cant_unidades_vendidas: integer;
  end;

  lista = ^nodo_venta;
  arbol = ^nodo;
  arbol_productos_vendidos = ^nodo_productos_vendidos;
  arbol_lista = ^nodo_lista;

  nodo = record
    prod: venta;
    HI: arbol;
    HD: arbol;
  end;

  nodo_lista = record
    dato: lista;
    HI: arbol_lista;
    HD: arbol_lista;
  end;

  nodo_venta = record
    dato: venta;
    sig: lista;
  end;

  nodo_productos_vendidos = record
    dato_venta: venta_record;
    HI: arbol_productos_vendidos;
    HD: arbol_productos_vendidos;
  end;

function fechaRandom():date;
var dt: date;
begin
  dt.yyyy:= random(High(rango_yyyy)); 
  dt.mm:= random(High(rango_mm)); 
  dt.dd:= random(High(rango_dd));

  while (dt.yyyy < Low(rango_yyyy)) do
      dt.yyyy:= random(High(rango_yyyy));

  while (dt.mm < Low(rango_mm)) do
    dt.mm:= random(High(rango_mm));

  while (dt.dd < Low(rango_dd))  do
    dt.dd:= random(High(rango_dd));
  fechaRandom:= dt;
end;

procedure LeerVenta(var v: venta);
begin
  v.cod_prod:= random(COD_PROD);
  v.fecha:= fechaRandom();
  v.cant_unidades_vendidas:= random(MAX_UNIDADES_VENDIDAS);
end;

procedure RecorrerArbol(a: arbol; var a_ventas: arbol_productos_vendidos; var a_lista: arbol_lista);      {inciso a(iii)}

  procedure InsertarVenta(var L:lista; v: venta);
  var aux: lista;
  begin
    new(aux);
    aux^.dato:= v;
    aux^.sig:= L;
    L:=aux;
  end;

  procedure InsertarArbolLista(v: venta; var a: arbol_lista);
  begin
    if a = nil then begin
      new(a);
      a^.HI:= nil;
      a^.HD:= nil;
      InsertarVenta(a^.dato, v);
    end else if a^.dato^.dato.cod_prod = v.cod_prod then
      InsertarVenta(a^.dato, v)
    else if a^.dato^.dato.cod_prod < v.cod_prod then
      InsertarArbolLista(v, a^.HI)
    else InsertarArbolLista(v, a^.HD);
  end;

  procedure CargarArbolLista(var a: arbol_lista; av: arbol);
  begin
    if a <> nil then begin
      CargarArbolLista(a^.HI, av);
      InsertarArbolLista(av^.prod, a);
      CargarArbolLista(a^.HD, av);
    end;
  end;

  procedure CargarArbolOrdenado(v: venta; var a: arbol_productos_vendidos);
  var aux: venta_record;
  begin
    if a = nil then begin
      new(a);
      a^.HI:= nil;
      a^.HD:= nil;
      aux.cod_prod:= v.cod_prod;
      aux.cant_unidades_vendidas:= v.cant_unidades_vendidas;
      a^.dato_venta:= aux;
    end else if a^.dato_venta.cod_prod = v.cod_prod then
      a^.dato_venta.cant_unidades_vendidas:= a^.dato_venta.cant_unidades_vendidas + v.cant_unidades_vendidas
    else if a^.dato_venta.cod_prod < v.cod_prod then
      CargarArbolOrdenado(v, a^.HI)
    else CargarArbolOrdenado(v, a^.HD);
  end;

begin
  if a <> nil then begin
    RecorrerArbol(a^.HI, a_ventas, a_lista);
    CargarArbolOrdenado(a^.prod, a_ventas);
    RecorrerArbol(a^.HD, a_ventas, a_lista);
  end;

end;

procedure AgregarArbol(var a: arbol; v: venta);
begin
  if a = nil then begin                                                 // es el primer elemento? 
    new(a);
    a^.prod:= v;
    a^.HI:= nil;
    a^.HD:= nil;
  end else if v.cod_prod < a^.prod.cod_prod then AgregarArbol(a^.HI, v) // es mas chico que en nodo?
  else AgregarArbol(a^.HD, v);                                          // entonces es mas grande
end;

procedure CargarArboles(var a: arbol; var a_ventas: arbol_productos_vendidos; var a_lista: arbol_lista);
var aux: venta;
begin
  LeerVenta(aux);
  while(aux.cod_prod <> 0) do begin   {inciso a(i)}
    AgregarArbol(a, aux);
    LeerVenta(aux);
  end;
  
  RecorrerArbol(a, a_ventas, a_lista);         {inciso a(ii)}
end;

function totalVentasEnFecha(a: arbol; d: date):integer;
  
  procedure ProductosEnFecha(a: arbol; var total: integer; d: date);
  begin
    if a <> nil then begin
      ProductosEnFecha(a^.HI, total, d);
      if (a^.prod.fecha.yyyy = d.yyyy) and (a^.prod.fecha.mm = d.mm) and (a^.prod.fecha.mm = d.mm) then
        total:= total + a^.prod.cant_unidades_vendidas;
      ProductosEnFecha(a^.HD, total, d);
    end;
  end;

var total: integer; 
begin
  total:=0;
  ProductosEnFecha(a, total, d);
  totalVentasEnFecha:=  total;
end;

procedure ContarVentas(a: arbol; var cant: integer);
begin
  if a <> nil then begin
    ContarVentas(a^.HI, cant);
    cant:=cant+1;
    ContarVentas(a^.HD, cant);
  end;
end;

function codMasUnidadesVendidas(a: arbol_productos_vendidos): integer;

  procedure ContarUnidadesVendidas(a: arbol_productos_vendidos; var mayor, total: integer);
  begin
    if a <> nil then begin
      ContarUnidadesVendidas(a^.HI, mayor, total);
      if a^.dato_venta.cant_unidades_vendidas > total then begin
        total:= a^.dato_venta.cant_unidades_vendidas;
        mayor:= a^.dato_venta.cod_prod;
      end;
      ContarUnidadesVendidas(a^.HD, mayor, total);
    end;
  end;

var mayor, total: integer;
begin
  total:= 0;
  mayor:= 0;
  ContarUnidadesVendidas(a, mayor, total);
  codMasUnidadesVendidas:= mayor;
end;

var a: arbol; a_ventas: arbol_productos_vendidos; a_lista: arbol_lista; fecha :date; cant: integer;
begin
  randomize;
  a:= nil;
  a_ventas:= nil;
  a_lista:= nil;
  cant:=0;
  fecha:= fechaRandom();
  CargarArboles(a, a_ventas, a_lista);
  ContarVentas(a, cant);
  writeln(#13#10'Cantidad de ventas: ', cant);
  writeln(#13#10'Cantidad ventas en la fecha ',fecha.dd,'/', fecha.mm,'/',fecha.yyyy,' : ', totalVentasEnFecha(a, fecha));        {inciso b}
  writeln(#13#10'Codigo de producto con mas unidades vendidas: ', codMasUnidadesVendidas(a_ventas));                              {inciso c}
end.

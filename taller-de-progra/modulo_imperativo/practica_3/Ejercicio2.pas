program Ejercicio_2;

type
  rango_yyyy = 1950..2024;
  rango_mm = 1..12;
  rango_dd = 1..31;
 
  date = record
    yyyy: integer;
    mm:   integer;
    dd:   integer;procedure CargarArbolOrdenado(var a: arbol_productos_vendidos);
begin
end;
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
  
  arbol = ^nodo;
  arbol_productos_vendidos = ^nodo_productos_vendidos;

  nodo = record
    prod: venta;
    HI: arbol;
    HD: arbol;
  end;

  nodo_productos_vendidos = record
    dato_venta: venta_record;
    HI: arbol_productos_vendidos;
    HD: arbol_productos_vendidos;
  end;

procedure LeerVenta(var v: venta);

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
begin
  v.cod_prod:= random(300);
  v.fecha:= fechaRandom();
  v.cant_unidades_vendidas:= random(50);
end;

procedure CargarArbolOrdenado(var a: arbol_productos_vendidos);
begin
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

procedure CargarArboles(var a: arbol; var a_ventas: arbol_productos_vendidos);
var aux: venta;
begin
  LeerVenta(aux);
  while(aux.cod_prod <> 0) do begin   {inciso a}
    AgregarArbol(a, aux);
    LeerVenta(aux);
  end;
    
  CargarArbolOrdenado(a_ventas, a);      {inciso b}
end;



var a: arbol; a_ventas: arbol_productos_vendidos;
begin
  randomize;
  a:= nil;
  a_ventas:= nil;
  CargarArboles(a, a_ventas);
end.
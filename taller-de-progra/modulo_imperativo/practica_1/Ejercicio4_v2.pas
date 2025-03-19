program Ejericio4;
const
    MAX_RUBRO = 8;
    COD_FIN = 0;
type
    rubroRange = 1..MAX_RUBRO;
    producto = record
        cod_producto: integer;
        cod_rubro: rubroRange;
        precio: real;
    end;
    lista = ^nodo;
    nodo = record
        dato: producto;
        sig: lista;
    end;
    vRubros = array [rubroRange] of lista;

procedure LeerProducto(var prod: producto);
begin
    readln(prod.precio);
    if (prod.precio <> 0) then begin
        readln(prod.cod_producto);
        readln(prod.cod_rubro);
    end;
end;

procedure InicializarListas(var vec: vRubros);
var
    i: integer;
begin
    for i:= 1 to MAX_RUBRO do begin
        vec[i]:=nil;
    end;
end;

procedure InsertarOrdenado(var L: lista; prod: producto);
var
    nue,act,ant: lista;
begin
    new(nue);
    act:=L;
    ant:=L;
    nue^.dato:=prod;
    while ((act <> ant) and (prod.cod_producto < act^.dato.cod_producto)) do begin
        ant:=act;
        act:=ant^.sig;
    end;
    if act = ant then
        L:=nue
    else
        ant^.sig:= nue;
    nue^.sig:= act;
end;

procedure CargarProductos(var vec: vRubros);
var
    prod: producto;
begin
    LeerProducto(prod);
    while prod.precio <> COD_FIN do begin
        InsertarOrdenado(vec[prod.cod_rubro], prod);
        LeerProducto(prod);
    end;
end;

var
    vec: vRubros;
begin
    InicializarListas(vec);
    CargarProductos(vec);
end.
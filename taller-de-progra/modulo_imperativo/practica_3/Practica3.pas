program Ejercicio_2_Practica_Arboles;

const
    FIN = 0;
    MAX_VENTAS = 20;
type
    rango_codProducto = 0..20;
    rango_fecha = 1..12;
    rango_cantVentas = 0..200;

    venta = record
        cod_producto: integer;
        fecha: integer;
        cant_vendidas: integer;
    end;
    
    arbol = ^nodoArbol;

    nodoArbol = record
        dato: venta;
        hi: arbol;
        hd: arbol;
    end;

procedure GenerarArbolRandom(var A:arbol);

    procedure InsertarVentaEnArbol(var A:arbol; e:venta);
    var nue: arbol;
    begin
        if (A = nil) then begin
            new(nue);
            nue^.dato:=e;
            nue^.hd:= nil;
            nue^.hi:= nil;
            A:= nue;
        end else begin
            if (A^.dato.cod_producto < e.cod_producto) then
                InsertarVentaEnArbol(A^.hd, e)
            else
                InsertarVentaEnArbol(A^.hi, e);
        end;
    end;
    
    function GenerarNumeroRandom(tipo: string):integer;
    var num:integer;
    begin
        if (tipo = 'codProducto') then begin
            num:= random(High(rango_codProducto));
            while(num < Low(rango_codProducto)) do
                num:= random(High(rango_codProducto));
            GenerarNumeroRandom:= num;
        end else if (tipo = 'fecha') then begin
            num:= random(High(rango_fecha));
            while(num < Low(rango_fecha)) do
                num:= random(High(rango_fecha));
            GenerarNumeroRandom:= num;
        end else begin
            num:= random(High(rango_cantVentas));
            while(num < Low(rango_cantVentas)) do
                num:= random(High(rango_cantVentas));
            GenerarNumeroRandom:= num;
        end;
    end;

var nue:venta;
begin
    with nue do begin
        cod_producto:=GenerarNumeroRandom('codProducto');
        fecha:=GenerarNumeroRandom('fecha');
        cant_vendidas:=GenerarNumeroRandom('cantVendidas');
    end;
    while nue.cod_producto <> 0 do begin
        with nue do begin
            cod_producto:=GenerarNumeroRandom('codProducto');
            fecha:=GenerarNumeroRandom('fecha');
            cant_vendidas:=GenerarNumeroRandom('cantVendidas');
        end;
        if (nue.cod_producto <> 0) then InsertarVentaEnArbol(A, nue);
    end;
end;

procedure ImprimirArbol(A:arbol);
begin
    if (A <> nil) then begin
        ImprimirArbol(A^.hi);
        write('COD: ', A^.dato.cod_producto, ', ');
        write('DATE: ', A^.dato.fecha, ', ');
        write('CANT: ', A^.dato.cant_vendidas);
        writeln();
        ImprimirArbol(A^.hd);
    end;
end;

procedure CargarArbolOrdenado(var B:arbol; A:arbol);
    
    procedure AgregarAlArbol(var B:arbol; v:venta);
    begin
        if (B = nil) then begin
            new(B);
            with B^ do begin
                dato.cod_producto:= v.cod_producto;
                dato.cant_vendidas:= v.cant_vendidas;
                hi:=nil;
                hd:=nil;
            end;
        end else if (B^.dato.cod_producto > v.cod_producto) then
            AgregarAlArbol(B^.hi, v)
        else
            B^.dato.cant_vendidas:= B^.dato.cant_vendidas + v.cant_vendidas;
    end;

begin
    if (A <> nil) then begin
        CargarArbolOrdenado(A^.hi, B);
        AgregarAlArbol(B,A^.dato);
        CargarArbolOrdenado(A^.hd, B);
    end;
end;

var A,B:arbol;
Begin
    GenerarArbolRandom(A);
    ImprimirArbol(A);
    CargarArbolOrdenado(A,B);
    writeln(B^.dato.cod_producto);
End.
{ Practica de Recursividad con opciones de ejecucion }

program Practica_Recuperatorio_Recursividad;

const
    FIN = 100;
type
    rango = 100..200;
    lista = ^nodo;
    nodo = record
        dato: integer;
        sig: lista;
    end;

procedure GenerarListaRandom(var L,Ult:lista; param:string);
    function NumeroRandom():integer;
    var n: integer;
    begin
        n:= random(High(rango));
        while(n < Low(rango)) do
            n:= random(High(rango));
        NumeroRandom:=n;
    end;

    procedure AgregarAdelante(var L:lista; num:integer);
    var nue: lista;
    begin
        new(nue);
        nue^.dato:=num;
        nue^.sig:=L;
        L:=nue;
    end;

    procedure AgregarAtras(var L, Ult:lista; num:integer);
    var nue: lista;
    begin
        new(nue);
        nue^.dato:=num;
        nue^.sig:=nil;
        if (L = nil) then
            L:= nue
        else
            Ult^.sig:= nue;
        Ult:=nue;
    end;

    procedure InsertarOrdenado(var L:lista; num:integer);
    var nue,act,ant:lista;
    begin
        new(nue); 
        act:=L; 
        ant:=L;
        nue^.dato:=num;
        while ((act <> nil) and (num < act^.dato)) do begin
            ant:= act;
            act:= act^.sig;
        end;
        if (act = ant) then
            L:=nue
        else ant^.sig:=nue;
        nue^.sig:=act;
    end;

var nue:integer;
begin
    nue:=NumeroRandom();
    while(nue <> FIN) do begin
        if (param = 'AgregarAtras') then
            AgregarAtras(L,Ult,nue)
        else if (param = 'AgregarAdelante') then
            AgregarAdelante(L,nue)
        else
            InsertarOrdenado(L,nue);
        nue:=NumeroRandom();
    end;
end;

procedure ImprimirLista(L:lista; param:string);
    procedure ImprimirListaRecursiva(L:lista);
    begin
        if (L <> nil) then begin
            writeln('> ', L^.dato);
            ImprimirListaRecursiva(L^.sig);
        end;
    end;
begin
    writeln('| Imprimiendo Lista de tipo: -', param);
    ImprimirListaRecursiva(L);
end;

procedure ImprimirListaInversa(L,Ult:lista; param:string);

    procedure ImprimirListaRecursivaInversa(L:lista);
    begin
        if (L <> nil) then begin
            ImprimirListaRecursivaInversa(L^.sig);
            writeln('> ', L^.dato);
        end;
    end;
begin
    writeln('| Imprimiendo Lista en orden inverso de tipo: -', param);
    ImprimirListaRecursivaInversa(L);
end;

procedure BuscarMinimoRecursivo(L:lista; var min:integer);
begin
    if (L <> nil) then begin
        if (L^.dato < min) then min:= L^.dato;
        BuscarMinimoRecursivo(L^.sig,min);
    end;
end;

function ExisteEnLaLista(L:lista; e:integer):boolean;
begin
    if (L = nil) then
        ExisteEnLaLista:= false
    else if (L^.dato = e) then
        ExisteEnLaLista:= true
    else
        ExisteEnLaLista:= ExisteEnLaLista(L^.sig,e);
end;

var L,Ult: lista; min:integer; param:string;
begin
    randomize;
    L:=nil;
    Ult:=nil;
    min:=300;

    { Parametros de ejecucion }

    param:= ParamStr(1);
    if (ParamCount = 0) then param:= 'InsertarOrdenado';
    //  Opciones validas: InsertarOrdenado, AgregarAtras, AgregarAdelante
    //  default: InsertarOrdenado
    
    GenerarListaRandom(L,Ult, param);
    ImprimirLista(L,param);
    ImprimirListaInversa(L,Ult, param);

    BuscarMinimoRecursivo(L,min);
    writeln('| Valor minimo: ', min);

    writeln('| Resultado de busqueda (', min, ') ', ExisteEnLaLista(L ,min));
end.
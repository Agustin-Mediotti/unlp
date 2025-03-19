program Ejercicio_3_Practica_Recuperatorio_Dios_meAyudeee;

const
    MAX = 20;
type
    rango = 300..1550;
    rango_vector = 1..MAX;
    vector = array[rango_vector] of rango;

procedure CargarVector(var arr:vector);

    function GenerarNumeroRandom():integer;
    var num:integer;
    begin
        num:= random(High(rango));
        while (num < Low(rango)) do
            num:= random(High(rango));
        GenerarNumeroRandom:=num;
    end;

var i: integer;
begin
    for i:=1 to MAX do
        arr[i]:=GenerarNumeroRandom();
end;

procedure OrdenarVector(var arr:vector);
var i,j,item,pos: integer;
begin
    for i:=1 to MAX -1 do begin
        pos:= i;
        for j:= i+1 to MAX do
            if arr[j] < arr[pos] then pos:= j;
        item:= arr[pos];
        arr[pos]:= arr[i];
        arr[i]:= item;
    end;
end;

procedure ImprimirVector(arr:vector);
var i: integer;
begin
    writeln('| Imprimiendo vector: ');
    for i:=1 to MAX do
        writeln('> ', arr[i]);
end;

var arr: vector;
begin
    randomize;
    CargarVector(arr);
    ImprimirVector(arr);
    OrdenarVector(arr);
    ImprimirVector(arr);
end.
program Ejercicio_3;

const MAX_ARR = 20;

type
  rango = 300..1550;
  vector = array[rango] of integer;
  indice = integer;

procedure CargarVector(var arr: vector);

  function GenerarNumeroRandom():integer;
  var num: integer;
  begin
    num:= random(High(rango));
    while num < Low(rango) do
      random(High(rango));
    GenerarNumeroRandom:= num;
  end;

  procedure CargarVectorRecursivo(var arr: vector; dimL: integer);
  begin
    if dimL <= MAX_ARR then begin
      arr[dimL]:= GenerarNumeroRandom();
      CargarVectorRecursivo(arr, dimL+1);
    end;
  end;

begin
  CargarVectorRecursivo(arr, 1);
end;

procedure OrdenarVector(var arr: vector);
var i,j,item,pos: integer;
begin
  for i:=1 to MAX_ARR -1 do begin
    pos:= i;
    for j:= i+1 to MAX_ARR do
      if arr[j] < arr[pos] then pos:= j;

    item:= arr[pos];
    arr[pos]:= arr[i];
    arr[i]:= item;
  end;
end;

procedure BusquedaDicotomica (arr: vector; ini,fin: indice; dato: integer; var pos: indice);
begin
  { TODO: El parámetro “pos” debe retornar la posición del dato o -1 si el dato no se encuentra
  en el vector. }
end;

var
  arr: vector;
  ini, fin, pos: indice;

begin
  ini:=0;
  fin:=0;
  randomize;

  CargarVector(arr);                              {inciso a}
  OrdenarVector(arr);                             {inciso b}
  BusquedaDicotomica(arr, ini, fin, 450, pos);    {inciso c}
end.

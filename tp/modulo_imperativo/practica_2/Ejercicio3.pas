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

procedure ImprimirVector(arr: vector; dl: integer);
begin
  if dl > 0 then begin
    writeln(arr[dl]);
    ImprimirVector(arr, dl-1);
  end;
end;

procedure BusquedaDicotomica (arr: vector; ini,fin: indice; dato: integer; var pos: indice);
var medio: integer;
begin
  { TODO: El parámetro “pos” debe retornar la posición del dato o -1 si el dato no se encuentra
  en el vector. }
  medio:= (ini+fin) div 2;
  
  if (arr[medio] = dato) then
    pos:= medio
  else begin
    if arr[medio] > dato then
      BusquedaDicotomica(arr, ini, medio, dato, pos)
    else if arr[medio] < dato then
      BusquedaDicotomica(arr, medio, fin, dato, pos)
      else
     pos:=-1;
 end;
end;

var
  arr: vector;
  ini, dato, pos: indice;

begin
  ini:=1;
  pos:=0;
  randomize;

  writeln('Dato a buscar: '); readln(dato);
  CargarVector(arr);                                   {inciso a}
  OrdenarVector(arr);                                  {inciso b}
  ImprimirVector(arr, MAX_ARR);
  BusquedaDicotomica(arr, ini, MAX_ARR, dato, pos);    {inciso c}
end.

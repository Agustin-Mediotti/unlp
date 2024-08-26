program ejercicio_8;

const dimF = 400;

type
  rango = 1..dimF;

  date = record
    day: integer;
    month: integer;
    year: integer;
  end;

  alumno = record
    n_insc: integer;
    dni: integer;
    apellido: string;
    nombre: string;
    nacimiento: date;
  end;

  alumnos = array [rango] of alumno;

function read_birthday():date;
var d: date;
begin
  readln(d.day);
  readln(d.month);
  readln(d.year);

  read_birthday:=d;
end;

procedure init_vec(var v: alumnos);
var i:integer;
begin
  for i:=1 to dimF do begin
    readln(v[i].n_insc);
    readln(v[i].dni);
    readln(v[i].apellido);
    readln(v[i].nombre);
    v[i].nacimiento:=read_birthday();
  end;
end;

function is_even(n: integer):boolean;
var aux, last: integer; even: boolean;
begin
  even:= true;
  aux:= n DIV 10;
  last:= n MOD 10;
  while((aux <> 0) and even) do begin
    if last MOD 2 <> 0 then
      even:=false;
    last:= aux MOD 10;
    aux:= aux DIV 10;
  end;
  is_even:= even;
end;

function percentil_even(v:alumnos):real;
var i, cant: integer;
begin
  cant:=0;
  for i:=1 to dimF do begin
    if is_even(v[i].dni) then
      cant:=cant+1;
  end;
    percentil_even:= (cant*100)/dimF;
end;

function date_to_days(d:date):integer; // no me importa calcular la edad exacta sino el mayor
begin
  date_to_days:= (((2024 - d.year)*365) + (d.month*28) + d.day);
end;

function elders(v:alumnos):string;
var 
  name1,name2: string;
  i, age: integer;
begin
  age:=-9999; name1:=''; name2:='';
  for i:=1 to dimF do begin
    if date_to_days(v[i].nacimiento) > age then begin
      age:= date_to_days(v[i].nacimiento);
      name2:=name1;
      name1:= v[i].nombre + ' ' + v[i].apellido;
    end;
  end;
  elders:= 'Los dos mayores son: ' + name1 + ' y ' + name2;
end;

var
  ingresantes: alumnos;

begin
  init_vec(ingresantes);
  writeln('el porcentaje de los alumnos con DNI de numeros pares es: ', percentil_even(ingresantes):0:2, '%');
  writeln(elders(ingresantes));
end.

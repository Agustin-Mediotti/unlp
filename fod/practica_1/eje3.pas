program ejercicio_3;

type
  empleado = record
    num_empleado: longint;
    nombre: string;
    apellido: string;
    edad: integer;
    dni: longint;
  end;
  archivo_empleados = file of empleado;

    {--------------------CARGA DE DATOS----------------------------} 

procedure CargarDatos(var archivo: archivo_empleados);
var emp: empleado;
begin
  rewrite(archivo);
  write('Apellido: '); readln(emp.apellido);
  while(emp.apellido <> 'fin') do begin
    write('Nombre: '); readln(emp.nombre);
    write('Edad: '); readln(emp.edad);
    write('DNI: '); readln(emp.dni);
    write('Num. de Empleado: '); readln(emp.num_empleado);
    write(archivo, emp);
    writeln('Apellido: '); readln(emp.apellido);
  end;
  close(archivo);
end;

    {--------------------BUSQUEDA DE REGISTRO----------------------------}

procedure BuscarEmpleado(var archivo: archivo_empleados);
var
  search: string;
  emp_actual: empleado;
begin    
  reset(archivo);
  writeln();
  write('Buscar empleado... '); readln(search);
  writeln();
  while(not EOF(archivo)) do begin
    read(archivo, emp_actual);
    if((emp_actual.nombre = search) or (emp_actual.apellido = search)) then begin
      write(emp_actual.nombre, ' ');
      write(emp_actual.apellido, ' ');
      write(emp_actual.edad, ' ');
      write(emp_actual.dni, ' ');
      writeln(emp_actual.num_empleado);
    end;
  end;
  close(archivo); 
end;

    {--------------------IMPRIMIR LISTA DE EMPLEADOS----------------------------}

procedure ImprimirLista(var archivo:archivo_empleados);
var emp_actual: empleado;
begin    
  reset(archivo);
  writeln();
  writeln('Imprimiendo archivo...');
  writeln();
  while(not EOF(archivo)) do begin
    read(archivo, emp_actual);
    write(emp_actual.nombre, ' ');
    write(emp_actual.apellido, ' ');
    write(emp_actual.edad, ' ');
    write(emp_actual.dni, ' ');
    writeln(emp_actual.num_empleado);
  end;
  close(archivo);
end;

    {--------------------LISTAR PROXIMOS A JUBILARSE----------------------------}

procedure ListarProximosJubilacion(var archivo: archivo_empleados);
var 
  por_jubilarse: integer;
  emp_actual: empleado;
begin
  por_jubilarse:=0;
  reset(archivo);
  writeln();
  while(not EOF(archivo)) do begin
    read(archivo, emp_actual);
    if(emp_actual.edad > 70) then
      por_jubilarse:=por_jubilarse+1;
  end;
  close(archivo);
  writeln('Próximos a jubilarse: ', por_jubilarse);
end;


    {----------------------------MENU UI--------------------------------}

procedure Menu(var archivo: archivo_empleados);
var 
  opcion:integer;
begin
  writeln();
  writeln('Ingrese una opción para continuar: ');
  writeln('1. Registro Nuevo    |   2. Buscar Empleado    |   3. Imprimir Lista   |   4. Edad Jubilatoria');
  readln(opcion);
  Case opcion of
    1 : CargarDatos(archivo);
    2 : BuscarEmpleado(archivo);
    3 : ImprimirLista(archivo);
    4 : ListarProximosJubilacion(archivo);
  end;
end;

var
  archivo: archivo_empleados;
  path: string;

begin
  write('Nombre del Archivo --> '); readln(path);
  assign(archivo, path);
  while (true) do
    Menu(archivo);
end.

program ejercicio_4;

type
  empleado = record
    num_empleado: longint;
    nombre: string;
    apellido: string;
    edad: integer;
    dni: longint;
  end;
  archivo_empleados = file of empleado;

    {--------------------CONTROL DE UNICIDAD----------------------------}

function VerificarNumeroEmpleado(numero: integer; var archivo: archivo_empleados): boolean;
var emp: empleado;
begin
  reset(archivo);
  while(not EOF(archivo)) do begin
    read(archivo, emp);
    if(emp.num_empleado = numero) then
      VerificarNumeroEmpleado:=true;
  end;
  close(archivo);
  VerificarNumeroEmpleado:=false;
end;

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

    {-------------------------AGREGAR EMPLEADO-------------------------------}

procedure AgregarEmpleado(var archivo: archivo_empleados);
var 
  opcion: string;
  emp: empleado;
begin
  opcion:='S';
  reset(archivo);
  writeln();
  seek(archivo, fileSize(archivo));
  while(opcion = 'S') do begin
    writeln('Apellido: '); readln(emp.apellido);
    write('Nombre: '); readln(emp.nombre);
    write('Edad: '); readln(emp.edad);
    write('DNI: '); readln(emp.dni);
    write('Num. de Empleado: '); readln(emp.num_empleado);
    while(VerificarNumeroEmpleado(emp.num_empleado, archivo)) do
      write('Num. de Empleado: '); readln(emp.num_empleado);
    write(archivo, emp);
    writeln();
    write('Queres seguir agregando registros? S/N '); readln(opcion);
  end;
end;
    {-------------------------MODIFICAR EMPLEADO-------------------------------}

procedure ModificarEmpleado(var archivo: archivo_empleados);
begin
//TODO
end;

    {------------------------------EXPORTAR----------------------------------------}

procedure ExportarLista(var archivo: archivo_empleados);
begin
//TODO
end;

    {-------------------------GENERAR REPORTE DNI-----------------------------}

procedure GenerarReporteFaltaDNI(var archivo: archivo_empleados);
begin
//TODO
end;

    {----------------------------MENU UI--------------------------------}

procedure Menu(var archivo: archivo_empleados);
var 
  opcion:integer;
begin
  writeln();
  writeln('Ingrese una opción para continuar: ');
  writeln('1. Registro Nuevo    |   2. Buscar Empleado    |   3. Imprimir Lista   |   4. Edad Jubilatoria');
  writeln('5. Agregar Empleado    |   6. Modificar Empleado    |   7. Exportar   |   8. Generar Reporte Sin DNI');
  readln(opcion);
  Case opcion of
    1 : CargarDatos(archivo);
    2 : BuscarEmpleado(archivo);
    3 : ImprimirLista(archivo);
    4 : ListarProximosJubilacion(archivo);
    5 : AgregarEmpleado(archivo);
    6 : ModificarEmpleado(archivo);
    7 : ExportarLista(archivo);
    8 : GenerarReporteFaltaDNI(archivo);
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

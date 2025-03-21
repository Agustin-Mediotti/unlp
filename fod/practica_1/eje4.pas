
Program ejercicio_4;

Const 
  ARCHIVO_TEXTO = 'todos_empleados.txt';
  ARCHIVO_REPORTE = 'faltaDNIEmpleado.txt';

Type 
  empleado = Record
    num_empleado: longint;
    nombre: string;
    apellido: string;
    edad: integer;
    dni: longint;
  End;
  archivo_empleados = file Of empleado;
  archivo_text = Text;

    {--------------------CONTROL DE UNICIDAD----------------------------}

Function VerificarNumeroEmpleado(numero: longint; Var archivo: archivo_empleados): boolean;

Var emp: empleado;
Begin
  reset(archivo);
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp);
      If (emp.num_empleado = numero) Then Begin
        close(archivo);
        VerificarNumeroEmpleado := false;
        Exit;
      End;
    End;
  close(archivo);
  VerificarNumeroEmpleado := true;
End;

    {--------------------BUSQUEDA INDICE DE REG.----------------------------}

Function BuscarIndice(Var archivo: archivo_empleados; num_empleado: integer): integer;

Var 
  emp: empleado;
Begin
  reset(archivo);
  While (Not EOF(archivo)) Do
    Begin
      BuscarIndice := filePos(archivo);
      read(archivo, emp);
      If (emp.num_empleado = num_empleado) Then
        Begin
          close(archivo);
          Exit;
        End;
    End;
  close(archivo);
  BuscarIndice := -1;
End;


    {--------------------CARGA DE DATOS----------------------------}

Procedure CargarDatos(Var archivo: archivo_empleados);

Var emp: empleado;
Begin
  rewrite(archivo);
  write('Apellido: '); readln(emp.apellido);
  While (emp.apellido <> 'fin') Do
    Begin
      write('Nombre: '); readln(emp.nombre);
      write('Edad: '); readln(emp.edad);
      write('DNI: '); readln(emp.dni);
      write('Num. de Empleado: '); readln(emp.num_empleado);
      write(archivo, emp);
      writeln('Apellido: '); readln(emp.apellido);
    End;
  close(archivo);
End;

    {--------------------BUSQUEDA DE REGISTRO----------------------------}

Procedure BuscarEmpleado(Var archivo: archivo_empleados);

Var 
  search: string;
  emp_actual: empleado;
Begin
  reset(archivo);
  writeln();
  write('Buscar empleado... ');
  readln(search);
  writeln();
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp_actual);
      If ((emp_actual.nombre = search) Or (emp_actual.apellido = search)) Then
        Begin
          write(emp_actual.nombre, ' ');
          write(emp_actual.apellido, ' ');
          write(emp_actual.edad, ' ');
          write(emp_actual.dni, ' ');
          writeln(emp_actual.num_empleado);
        End;
    End;
  close(archivo);
End;


{--------------------IMPRIMIR LISTA DE EMPLEADOS----------------------------}

Procedure ImprimirLista(Var archivo:archivo_empleados);

Var emp_actual: empleado;
Begin
  reset(archivo);
  writeln();
  writeln('Imprimiendo archivo...');
  writeln();
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp_actual);
      write(emp_actual.nombre, ' ');
      write(emp_actual.apellido, ' ');
      write(emp_actual.edad, ' ');
      write(emp_actual.dni, ' ');
      writeln(emp_actual.num_empleado);
    End;
  close(archivo);
End;


{--------------------LISTAR PROXIMOS A JUBILARSE----------------------------}

Procedure ListarProximosJubilacion(Var archivo: archivo_empleados);

Var 
  por_jubilarse: integer;
  emp_actual: empleado;
Begin
  por_jubilarse := 0;
  reset(archivo);
  writeln();
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp_actual);
      If (emp_actual.edad > 70) Then
        por_jubilarse := por_jubilarse+1;
    End;
  close(archivo);
  writeln('Próximos a jubilarse: ', por_jubilarse);
End;

    {-------------------------AGREGAR EMPLEADO-------------------------------}

Procedure AgregarEmpleado(Var archivo: archivo_empleados);

Var 
  opcion: string;
  emp: empleado;
  ok: boolean;
Begin
  opcion := 'S';
  writeln();
  While (opcion = 'S') Do
    Begin
      writeln('Apellido: '); readln(emp.apellido);
      write('Nombre: '); readln(emp.nombre);
      write('Edad: '); readln(emp.edad);
      write('DNI: '); readln(emp.dni);
      write('Num. de Empleado: '); readln(emp.num_empleado);
      
      ok := VerificarNumeroEmpleado(emp.num_empleado, archivo);
      While (not ok) Do
        Begin
          Write('Num. de Empleado: '); readln(emp.num_empleado);
          ok := VerificarNumeroEmpleado(emp.num_empleado, archivo);
        End;
      reset(archivo);
      seek(archivo, fileSize(archivo));
      write(archivo, emp);
      close(archivo);
      writeln();
      write('Queres seguir agregando registros? S/N ');
      readln(opcion);
    End;
End;
    {-------------------------MODIFICAR EMPLEADO-------------------------------}

Procedure ModificarEmpleado(Var archivo: archivo_empleados);

Var 
  num_empleado, indice, nueva_edad: integer;
  emp: empleado;
  opcion: string;
Begin
  write('Ingrese el Numero de Empleado...');
  readln(num_empleado);
  indice := BuscarIndice(archivo, num_empleado);
  If (indice <> -1) Then
    Begin
      reset(archivo);
      seek(archivo, indice);
      read(archivo, emp);
      writeln('Seleccionado: ', emp.nombre, ' ', emp.apellido, ', Edad: ', emp.
              edad);
      write('Modificar edad: ');
      readln(nueva_edad);
      write(emp.edad, ' -> ', nueva_edad, '. Confirmar cambios? S/N ');
      readln(opcion);
      If (opcion = 'S') Then
      begin
        seek(archivo, indice);
        emp.edad:=nueva_edad;
        write(archivo, emp);
      end;
      close(archivo);
    End;
End;


{------------------------------EXPORTAR----------------------------------------}

Procedure ExportarLista(Var archivo: archivo_empleados);

Var 
  archivo_export: archivo_text;
  emp: empleado;
Begin
  reset(archivo);
  assign(archivo_export, ARCHIVO_TEXTO);
  rewrite(archivo_export);
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp);
      writeln(archivo_export, 'Nombre: ', emp.nombre, ' Apellido: ', emp.apellido,
      ' DNI: ', emp.dni, ' Edad: ', emp.edad,' Num. Empleado: ', emp.num_empleado);
    End;
  close(archivo);
  close(archivo_export);
  writeln('Fichero Creado: ', ARCHIVO_TEXTO);
End;

    {-------------------------GENERAR REPORTE DNI-----------------------------}

Procedure GenerarReporteFaltaDNI(Var archivo: archivo_empleados);

Var 
  reporte_export: archivo_text;
  emp: empleado;
Begin
  reset(archivo);
  assign(reporte_export, ARCHIVO_REPORTE);
  rewrite(reporte_export);
  While (Not EOF(archivo)) Do
    Begin
      read(archivo, emp);
      If (emp.dni = 00) then
        Begin
          writeln(reporte_export, 'Nombre: ', emp.nombre, ' Apellido: ', emp.
              apellido,
              ' DNI: ', emp.dni, ' Edad: ', emp.edad,' Num. Empleado: ', emp.
              num_empleado);
        End;
    End;
  close(archivo);
  close(reporte_export);
  writeln('Fichero Creado: ', ARCHIVO_REPORTE);
End;


    {----------------------------MENU TUI--------------------------------}

Procedure Menu(Var archivo: archivo_empleados);

Var 
  opcion: integer;
Begin
  writeln();
  writeln('Ingrese una opción para continuar: ');
  writeln('1. Registro Nuevo    |   2. Buscar Empleado    |   3. Imprimir Lista   |   4. Edad Jubilatoria');
  writeln('5. Agregar Empleado    |   6. Modificar Empleado    |   7. Exportar   |   8. Generar Reporte Sin DNI');
  readln(opcion);
  Case opcion Of 
    1 : CargarDatos(archivo);
    2 : BuscarEmpleado(archivo);
    3 : ImprimirLista(archivo);
    4 : ListarProximosJubilacion(archivo);
    5 : AgregarEmpleado(archivo);
    6 : ModificarEmpleado(archivo);
    7 : ExportarLista(archivo);
    8 : GenerarReporteFaltaDNI(archivo);
  End;
End;

Var 
  archivo: archivo_empleados;
  path: string;

Begin
  write('Nombre del Archivo --> '); readln(path);
  assign(archivo, path);
  While (true) Do
    Menu(archivo);
End.

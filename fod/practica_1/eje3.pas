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

  var
    archivo: archivo_empleados;
    path: string;
    search: string;
    por_jubilarse: integer;
    emp: empleado;
    emp_actual: empleado;

  begin
    por_jubilarse:=0;
    write('Nombre del archivo --> '); readln(path);
    assign(archivo, path);
    
    {--------------------CARGA DE DATOS----------------------------} 

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

    {--------------------BUSQUEDA DE REGISTRO----------------------------}
    
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

    {--------------------IMPRIMIR LISTA DE EMPLEADOS----------------------------}

    reset(archivo);
    writeln();
    writeln('Imprimiendo archivo...');
    writeln();
    while(not EOF(archivo)) do begin
      read(archivo, emp_actual);
      if(emp_actual.edad > 70) then
        por_jubilarse:=por_jubilarse+1;
       write(emp_actual.nombre, ' ');
       write(emp_actual.apellido, ' ');
       write(emp_actual.edad, ' ');
       write(emp_actual.dni, ' ');
       writeln(emp_actual.num_empleado);
     end;
    close(archivo);

    {--------------------LISTAR PROXIMOS A JUBILARSE----------------------------}
    
    writeln();
    writeln('Próximos a jubilarse: ', por_jubilarse);
end.

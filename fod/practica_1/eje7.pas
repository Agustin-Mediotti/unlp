
Program ejercicio_7;

Uses sysutils; { Para dar correcto formato al archivo de texto. }

Const 
  TEXT_FILE = 'novelas.txt';

Type 
   novela = Record
     codigo : longint;
     nombre : string;
     genero : string;
     precio : double;
   End;
   binario =  File Of novela;

Procedure ImportarArchivo(Var archivo : binario; Var textFile : text);
Var
   nov  : novela;
   cant : integer;
Begin
   cant:=0;
   While(not EOF(textFile)) Do
      Begin
         readln(textFile, nov.codigo, nov.precio, nov.genero);
         readln(textFile, nov.nombre);
         write(archivo, nov);
         cant:= cant+1;
      End;
   writeln();
   writeln('Archivo ', TEXT_FILE, ' importado. Se agregaron ', cant, ' registros.');
End;

Procedure GuardarCambios(Var archivo : binario; Var textFile : text);
Var
   cant : integer;
   nov  : novela;
Begin
   cant:=0;
   reset(archivo);
   rewrite(textFile);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, nov);
         writeln(textFile, IntToStr(nov.codigo) + ' ' + FloatToStr(nov.precio) + ' ' + nov.genero);
         writeln(textFile, nov.nombre);
         cant:= cant+1;
      End;
   close(textFile);
   close(archivo);
   writeln();
   writeln('Se actualizaron ', cant, ' registros.');
End;

Procedure MostrarArchivo(Var archivo : binario);
Var
   nov : novela;
Begin
   reset(archivo);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, nov);
         writeln();
         writeln('> ', nov.nombre, ' : C-', nov.codigo);
         writeln(' ', nov.genero);
         writeln('  $', nov.precio:0:2);
      End;
   close(archivo);
End;

Procedure ModificarRegistro(Var archivo : binario; Var textFile : text);
   Procedure EditarNombre(Var nov : novela);
   Var
      nombre, opcion : string;
   Begin
      write('Nombre: ');
      readln(nombre);
      write(nov.nombre, ' >> ', nombre, ' || Aceptar Cambios? Y/N ');
      readln(opcion);
      If(opcion = 'Y') Then
         nov.nombre:= nombre;
   End;
   Procedure EditarGenero(Var nov : novela);
   Var
      genero, opcion : string;
   Begin
      write('Genero: ');
      readln(genero);
      write(nov.genero, ' >> ', genero, ' || Aceptar Cambios? Y/N ');
      readln(opcion);
      If(opcion = 'Y') Then
         nov.genero:= genero;
   End;
   Procedure EditarPrecio(Var nov : novela);
   Var
      opcion : string;
      precio : double;
   Begin
      write('Precio: ');
      readln(precio);
      write(nov.precio:0:2, ' >> ', precio:0:2, ' || Aceptar Cambios? Y/N ');
      readln(opcion);
      If(opcion = 'Y') Then
         nov.precio:= precio;
   End;
   Procedure ImprimirRegistro(nov : novela);
   Begin
      writeln();
      writeln('> ', nov.nombre, ' : C-', nov.codigo);
      writeln(' ', nov.genero);
      writeln('  $', nov.precio:0:2);
   End;
Var
   codigo, opcion : integer;
   nov            : novela;
   ok             : boolean;
Begin
   reset(archivo);
   writeln();
   write('Codigo: ');
   readln(codigo);
   ok:=false;
   While((not EOF(archivo)) and (not ok)) Do
      Begin
         read(archivo, nov);
         If(nov.codigo = codigo) Then
            Begin
               ok:=true;
               opcion:=-1;
               While(opcion <> 0) Do
                  Begin
                     ImprimirRegistro(nov);
                     writeln();
                     writeln('1. Editar Nombre  |  2. Editar Genero  |  3. Editar Precio  |  0. Guardar y Salir');
                     readln(opcion);
                     Case opcion Of
                       1 : EditarNombre(nov);
                       2 : EditarGenero(nov);
                       3 : EditarPrecio(nov);
                       0 : writeln();
                     End;
                  End;
            End;
      End;
   close(archivo);
   If(ok) Then
      GuardarCambios(archivo, textFile)
   Else
      writeln('No se encontro el registro.');
End;

Procedure AgregarRegistro(Var archivo : binario; Var textFile : text);
   Procedure LeerRegistro(Var nov : novela);
   Begin
      writeln();
      write('Nombre: ');
      readln(nov.nombre);
      write('Codigo: ');
      readln(nov.codigo);
      write('Genero: ');
      readln(nov.genero);
      write('Precio: ');
      readln(nov.precio);
   End;
Var
   nov : novela;
Begin
   reset(archivo);
   writeln('Nuevo Registro');
   seek(archivo, fileSize(archivo));
   LeerRegistro(nov);
   write(archivo, nov);
   close(archivo);
   GuardarCambios(archivo, textFile);
End;

Procedure MenuUI(Var archivo : binario; Var textFile : text);
Var
   option : integer;
Begin
   writeln();
   writeln('1. Imprimir Archivo  |  2. Modificar Registro  |  3. Agregar Registro');
   write('Ingrese una opción para continuar: ');
   readln(option);
   Case option Of
     1 : MostrarArchivo(archivo);
     2 : ModificarRegistro(archivo, textFile);
     3 : AgregarRegistro(archivo, textFile);
   End;
End;

Var
   path     : string;
   archivo  : binario;
   textFile : text;
Begin
   write('Archivo binario --> ');
   readln(path);
   assign(archivo, path);
   assign(textFile, TEXT_FILE);
   rewrite(archivo);
   reset(textFile);
   ImportarArchivo(archivo, textFile);
   While(true) Do
      MenuUI(archivo, textFile);
End.

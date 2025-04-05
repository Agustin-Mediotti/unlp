
Program ejercicio_7;

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
   texto = Text;

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

Procedure MostrarArchivo(Var archivo : binario);
Var
   nov : novela;
Begin
   reset(archivo);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, nov);
         writeln();
         writeln('> ', nov.nombre, ' : C', nov.codigo);
         writeln(' ', nov.genero);
         writeln('  $', nov.precio:0:2);
      End;
   close(archivo);
End;

Procedure ModificarRegistro(Var archivo : binario);
Begin
   writeln('TODO');
End;

Procedure AgregarElemento(Var archivo : binario);
Begin
   writeln('TODO');
End;

Procedure MenuUI(Var archivo : binario);
Var
   option : integer;
Begin
   writeln();
   writeln('1. Imprimir Archivo  |  2. Modificar Registro  |  3. Agregar Registro');
   write('Ingrese una opción para continuar: ');
   readln(option);
   Case option Of
     1 : MostrarArchivo(archivo);
     2 : ModificarRegistro(archivo);
     3 : AgregarElemento(archivo);
   End;
End;

Var
   path     : string;
   archivo  : binario;
   textFile : texto;
Begin
   write('Archivo binario --> ');
   readln(path);
   assign(archivo, path);
   assign(textFile, TEXT_FILE);
   rewrite(archivo);
   reset(textFile);
   ImportarArchivo(archivo, textFile);
   While(true) Do
      MenuUI(archivo);
End.


Program ejercicio_5;

Const 
  IMPORT_EXPORT = 'celulares.txt';

Type 
  celulares = Record
    codigo: longint;
    nombre: string;
    descripcion: string;
    marca: string;
    precio: double;
    stock_min: integer;
    stock_disponible: integer;
  End;
  texto = Text;
  binario = file Of celulares;

    {---------------------EXPORTAR DATOS A TXT---------------------}

Procedure Exportar(Var archivo : binario);
Var
   textFile : Text;
   celular  : celulares;
Begin
   reset(archivo);
   assign(textFile, IMPORT_EXPORT);
   rewrite(textFile);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, celular);
         writeln(textFile, celular.codigo, ' ', celular.precio, ' ', celular.marca);
         writeln(textFile, celular.stock_disponible, ' ', celular.stock_min, ' ', celular.descripcion);
         writeln(textFile, celular.nombre);
      End;
   close(textFile);
   close(archivo);
End;

    {---------------------MOSTRAR DESCRIPCION CUSTOM---------------------}

Procedure ListarCustomDescripcion(Var archivo : binario);
Var
   celular : celulares;
Begin
   reset(archivo);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, celular);
         If(celular.descripcion <> '') Then
            writeln('> ', celular.marca, ' ', celular.nombre, ' C', celular.codigo,
                    ' Descripcion: ', celular.descripcion);
      End;
   close(archivo);
End;

    {---------------------MOSTRAR STOCK MINIMO---------------------}

Procedure ListarStockMinimo(Var archivo: binario);
Var
   celular :  celulares;
Begin
   reset(archivo);
   While(not EOF(archivo)) Do
      Begin
         read(archivo, celular);
         If(celular.stock_disponible < celular.stock_min) Then
            writeln('> ', celular.marca, ' ', celular.nombre, ' C', celular.codigo, ' $', celular.precio:0:2,
                    ' stock/stock min: ', celular.stock_disponible, '/', celular.stock_min);
      End;
   close(archivo);
End;


    {-------------------------CARGA DE DATOS-------------------------}

Procedure ImportarDatos(Var archivo : binario);
Var
   textFile : Text;
   celular  : celulares;
   reg      :  integer;
Begin
   reset(archivo);
   assign(textFile, IMPORT_EXPORT);
   reset(textFile);
   reg:=0;
   while(not EOF(textFile)) Do
      Begin
         readln(textFile, celular.codigo, celular.precio, celular.marca);
         readln(textFile, celular.stock_disponible, celular.stock_min, celular.descripcion);
         readln(textFile, celular.nombre);
         write(archivo, celular);
         reg:= reg+1;
      End;
   close(textFile);
   close(archivo);
   writeln();
   writeln('Archivo ', IMPORT_EXPORT, ' se han importado exitosamente ', reg, ' registros.');
End;

    {-------------------------MENU TUI-------------------------}

Procedure MenuUi(Var archivo: binario);
Var
   opcion : integer;
Begin
   writeln();
   writeln('Ingrese una opción para continuar: ');
   writeln('1. Importar Datos  |  2. Listar Stock Mínimo  |  3. Listar Descripción  |  4. Exportar Archivo');
   readln(opcion);
  Case opcion Of 
    1 : ImportarDatos(archivo);
    2 : ListarStockMinimo(archivo);
    3 : ListarCustomDescripcion(archivo);
    4 : Exportar(archivo);
  End;
End;

Var 
  path: string;
  archivo: binario;
Begin
  write('Nombre del archivo --> ');
  readln(path);
  assign(archivo, path);
  rewrite(archivo);
  While (true) Do
    MenuUI(archivo);
End.


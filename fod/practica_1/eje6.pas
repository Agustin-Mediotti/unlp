
Program ejercicio_6;

Const 
  IMPORT_EXPORT    = 'celulares.txt';
  IMPORT_SIN_STOCK =  'SinStock.txt';

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

Function BuscarRegistro(Var archivo : binario; nombre : string):boolean;
Var
   ok      : boolean;
   celular :  celulares;
Begin
   ok:=false;
   While((not EOF(archivo)) and (not ok)) Do
      Begin
         read(archivo, celular);
         ok:= celular.nombre = nombre;
      End;
   If(ok) Then
      Begin
         seek(archivo, filePos(archivo) - 1);
         BuscarRegistro:=true;
      End
   Else
      Begin
         seek(archivo, 0);
         BuscarRegistro:=false;
      End;
End;
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
   writeln();
   writeln('Archivo guardado en ', IMPORT_EXPORT);
End;

    {---------------------MOSTRAR DESCRIPCION CUSTOM---------------------}

Procedure ListarCustomDescripcion(Var archivo : binario);
Var
   celular : celulares;
Begin
   reset(archivo);
   writeln();
   While(not EOF(archivo)) Do
      Begin
         read(archivo, celular);
         If(celular.descripcion <> '') Then
            writeln('> ', celular.marca, ' ', celular.nombre, ' C', celular.codigo,
                    ' Descripcion: ', celular.descripcion);
      End;
   close(archivo);
End;

    {---------------------AGREGAR ELEMENTO A BINARIO---------------------}

Procedure AgregarCelular(Var archivo : binario);
   Procedure NuevoRegistro(Var celular : celulares);
   Begin
      writeln();
      write('Codigo: ');
      readln(celular.codigo);
      write('Marca ');
      readln(celular.marca);
      write('Nombre (Modelo): ');
      readln(celular.nombre);
      write('Precio: ');
      readln(celular.precio);
      write('Stock: ');
      readln(celular.stock_disponible);
      write('Stock Mínimo: ');
      readln(celular.stock_min);
      write('Descripción: ');
      readln(celular.descripcion);
   End;
Var
   ok      : string;
   celular : celulares;
Begin
   reset(archivo);
   ok:='Y';
   While(ok = 'Y') Do
      Begin
         NuevoRegistro(celular);
         seek(archivo, fileSize(archivo));
         write(archivo, celular);
         writeln();
         write('Agregar nuevo elemento? Y/N ');
         readln(ok);
      End;
   close(archivo);
End;

    {---------------------MODIFICAR STOCK DE ELEMENTO---------------------}

Procedure ModificarStock(Var archivo : binario);
Var
   nombre, opcion : string;
   callback       : boolean;
   celular        : celulares;
   stock, indice  : integer;
Begin
   reset(archivo);
   callback:=false;
   writeln();
   write('Nombre de Celular: ');
   readln(nombre);
   callback:= BuscarRegistro(archivo, nombre);
   If(callback) Then
   Begin
      indice:= filePos(archivo);
      read(archivo, celular);
      writeln();
      writeln(celular.nombre, ' ', celular.marca, ': stock actual ', celular.stock_disponible);
      write('Nuevo monto: ');
      readln(stock);
      writeln(celular.stock_disponible, ' >> ', stock, ' Aceptar cambios? Y/N');
      readln(opcion);
      If(opcion = 'Y') Then
      Begin
         celular.stock_disponible:= stock;
         seek(archivo, indice);
         write(archivo, celular);
         writeln();
         writeln('Registro binario actualizado.');
      End;
   End
Else
   writeln('> ERROR: El registro no se encontro en el archivo binario.');
   close(archivo);
End;

    {---------------------AGREGAR ELEMENTO A BINARIO---------------------}

Procedure ExportarSinStock(Var archivo : binario);
Var
   textFile : Text;
   celular  : celulares;
   cant     : integer;
Begin
   reset(archivo);
   assign(textFile, IMPORT_SIN_STOCK);
   rewrite(textFile);
   cant:=0;
   While(not EOF(archivo)) Do
      Begin
         read(archivo, celular);
         If(celular.stock_disponible = 0) Then
            Begin
               writeln(textFile, celular.codigo, ' ', celular.precio,' ', celular.marca);
               writeln(textFile, celular.stock_disponible, ' ', celular.stock_min, ' ', celular.descripcion);
               writeln(textFile, celular.nombre);
               cant:= cant+1;
            End;
      End;
   close(textFile);
   close(archivo);
   writeln();
   writeln('Se genero el archivo ', IMPORT_SIN_STOCK,' con ', cant, ' registros.');
End;

    {---------------------MOSTRAR STOCK MINIMO---------------------}

Procedure ListarStockMinimo(Var archivo: binario);
Var
   celular :  celulares;
Begin
   reset(archivo);
   writeln();
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
   writeln('1. Importar Datos  |  2. Listar Stock Mínimo  |  3. Listar Descripción  |  4. Agregar Celular');
   writeln('5. Modificar Stock  |  6. Generar Reporte Sin Stock  |  7. Guardar Cambios');
   readln(opcion);
  Case opcion Of 
    1 : ImportarDatos(archivo);
    2 : ListarStockMinimo(archivo);
    3 : ListarCustomDescripcion(archivo);
    4 : AgregarCelular(archivo);
    5 : ModificarStock(archivo);
    6 : ExportarSinStock(archivo);
    7 : Exportar(archivo);
  End;
End;

Var 
  path: string;
  archivo: binario;
Begin
  write('Nombre del archivo binario --> ');
  readln(path);
  assign(archivo, path);
  rewrite(archivo);
  While (true) Do
    MenuUI(archivo);
End.


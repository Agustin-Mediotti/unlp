
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
    stock_dispomible: integer;
  End;
  texto = Text;
  binario = file Of celulares;

Procedure MenuUi(Var archivo: binario);
Begin
  Case opcion Of 
    1 : ImportarDatos(archivo);
    2 : ListarStockMinimo(archivo);
    3 : ListarCustomDescripcion(archivo);
    4 : Exportar(archivo);
  End;

Var 
  path: string;
  archivo: binario;
Begin
  write('Nombre del archivo --> ');
  readln(path);
  assign(archivo, path);
  While (true) Do
    MenuUI(archivo);
End.


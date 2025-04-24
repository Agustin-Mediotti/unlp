Program ejercicio_2;

Const
   VALOR_ALTO   = 'ZZZZ';
   PATH_MAESTRO = 'maestro';
   PATH_DETALLE = 'detalle';
   PATH_TXT     = 'stock_minimo.txt';

Type
   str4     = string[4];
   producto = record
      cod     : str4;
      nombre  : string[20];
      pv      : double;
      stock   : integer;
      stk_min : integer;
   end;
   venta = record
      cod   : str4;
      total : integer;
   end;
   detalle  = file of venta;
   maestro  = file of producto;

Procedure Leer(Var det : detalle; Var reg : venta);
Begin
   If(not EOF(det)) Then
      read(det, reg)
   Else
      reg.cod:= VALOR_ALTO;
End;

Procedure Actualizar(Var mae : maestro; Var det : detalle);
Var
   regd  : venta;
   regm  : producto;
   aux   : str4;
   total : integer;
Begin
   reset(det);
   reset(mae);
   Leer(det, regd);
   read(mae, regm);
   While(regd.cod <> VALOR_ALTO) Do
      Begin
         aux:=regd.cod;
         total:=0;

         While(aux = regd.cod) Do
            Begin
               total:= total + regd.total;
               Leer(det, regd);
            End;

         While(regm.cod <> aux) Do
            read(mae, regm);
         regm.stock:= regm.stock - total;
         seek(mae, filePos(mae)-1);
         write(mae, regm);
         If(not EOF(mae)) Then
            read(mae, regm);
      End;
   close(det);
   close(mae);
End;

Procedure Exportar(Var mae : maestro);
Var
   export : text;
   regm   : producto;
Begin
   reset(mae);
   assign(export, PATH_TXT);
   rewrite(export);
   While(not EOF(mae)) Do
      Begin
         read(mae, regm);
         If(regm.stock < regm.stk_min) Then
            writeln(export, regm.cod, ' ', regm.nombre, ' ', regm.pv:0:2);
      End;
   close(export);
   close(mae);
End;

Procedure Menu(Var mae : maestro; Var det : detalle);
Var
   op : integer;
Begin
   writeln();
   writeln('1. Actualizar archivo maestro  |  2. Exportar stock minimo');
   write('Elije una opcion: ');
   readln(op);
   Case op Of
     1 : Actualizar(mae, det);
     2 : Exportar(mae);
   End;
End;

Var
   mae : maestro;
   det : detalle;

Begin
   assign(mae, PATH_MAESTRO);
   assign(det, PATH_DETALLE);
   While(true) Do
      Menu(mae, det);
End.

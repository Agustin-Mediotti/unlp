Program ejercicio_1;

Const
   ARCHIVO_COMISIONES = 'comisiones';
   VALOR_ALTO         =  32767;

Type
   comision = record
      codigo_empleado : integer;
      nombre          : string;
      monto_comision  : double;
   end;

   comisiones_f = File of comision;

Procedure Leer(Var comisiones : comisiones_f; Var empleado : comision);
Begin
   If(not EOF(comisiones)) Then
      read(comisiones, empleado)
    Else
       empleado.codigo_empleado:= VALOR_ALTO;
End;

Procedure ProcesarArchivo(Var comisiones : comisiones_f);
Var
   comisiones_compacto : comisiones_f;
   empleado_actual     : comision;
   empleado_total      : comision;

Begin
   rewrite(comisiones_compacto);
   Leer(comisiones, empleado_actual);
   While(empleado_actual.codigo_empleado <> VALOR_ALTO) Do
      Begin
         empleado_total:= empleado_actual;
         While(empleado_actual.codigo_empleado = empleado_total.codigo_empleado) Do
            Begin
               empleado_total.monto_comision:= empleado_total.monto_comision + empleado_actual.monto_comision;
               Leer(comisiones, empleado_actual);
            End;
         write(comisiones_compacto, empleado_total);
      End;
   close(comisiones_compacto);
End;

Var
   comisiones : comisiones_f;

Begin
   assign(comisiones, ARCHIVO_COMISIONES);
   reset(comisiones);
   ProcesarArchivo(comisiones);
   close(comisiones);
End.

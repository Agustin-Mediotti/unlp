{
 *
 *  Merge:
 *  Proceso de generacion de un nuevo archivo a partir de otros existentes.
 *
 *  - La informacion se encuentra ordenada por el mismo criterio en cada caso.
 *  - En este caso el archivo maestro ya existe.
 *
 *  Ref. Introduccion a las Bases de Datos, pag 44.
}

Program ejercicio_3;

Const
   VALOR_ALTO = 'ZZZZ';
   ARC_MAESTRO    = 'archivo_maestro';
   ARC_DETALLE_1  = 'archivo_detalle_01';
   ARC_DETALLE_2  = 'archivo_detalle_02';

Type
   provincia   =
   record
      nombre: string[30];
      cant_alf: integer;
      cant_enc : integer;
   end;
   datos_censo =
   record
      nombre   : string[30];
      cod      : string[4];
      cant_alf : integer;
      cant_enc : integer;
   end;
   maestro     = file of provincia;
   detalle     = file of datos_censo;

(* Leemos el siguiente elemento o retornamos el codigo fuera de rango *)
Procedure Leer(Var det : detalle; Var reg : datos_censo);
Begin
   If(not(EOF(det))) Then
      read(det, reg)
   Else
      reg.cod:= VALOR_ALTO;
End;

(* Busca el registro minimo segun criterio de orden *)
Procedure Minimo(Var det1, det2 : detalle; Var min, reg1, reg2 : datos_censo);
Begin
   If (reg1.nombre < reg2.nombre) Then
      Begin
         min.nombre:= reg1.nombre;
         min.cant_enc:= reg1.cant_enc;
         min.cant_alf:= reg1.cant_alf;
         read(det1, reg1);
      End
   Else Begin
      min.nombre:= reg2.nombre;
      min.cant_enc:= reg2.cant_enc;
      min.cant_alf:= reg2.cant_alf;
      read(det2, reg2);
   End;
End;

Procedure Actualizar(Var mae: maestro; Var det1,det2 : detalle);
Var
   reg1, reg2, min      : datos_censo;
   regm                 : provincia;
   total_alf, total_enc : integer;
   cod_act              : string[4];
Begin
   read(det1, reg1);
   read(det2, reg2);
   read(mae, regm);
   Minimo(det1, det2, min, reg1, reg2);
   cod_act:=min.cod;

   { Leemos todos los elementos de los detalles }

   While(min.cod <> VALOR_ALTO) Do
      Begin

         { Totalizamos los valores a actualizar }

         total_alf:=0;
         total_enc:=0;
         While(min.cod = cod_act) Do
            Begin
               total_alf:= total_alf + min.cant_alf;
               total_enc:= total_enc + min.cant_enc;
               Minimo(det1, det2, min, reg1, reg2);
            End;

         { Buscamos el registro en el maestro }

         While(regm.nombre <> min.nombre) Do
            read(mae, regm);

         { Actualizamos los campos en el reg. local }

         regm.cant_alf:= regm.cant_alf + total_alf;
         regm.cant_enc:= regm.cant_enc + total_enc;

         { Guardamos cambios en el maestro }

         seek(mae, filepos(mae)-1);
         write(mae, regm);

         { Actualizamos el codigo actual }

         cod_act:= min.cod;
      End;
   close(mae);
   close(det1);
   close(det2);
End;

Var
   mae  : maestro;
   det1,det2 : detalle;
Begin
   assign(mae, ARC_MAESTRO);
   assign(det1, ARC_DETALLE_1);
   assign(det2, ARC_DETALLE_2);
   reset(mae);                  // El ejercicio indica que el archivo maestro EXISTE.
   Actualizar(mae, det1, det2);
End.

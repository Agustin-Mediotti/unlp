# El siguiente programa recorre un arreglo de números (TABLA1) y genera un segundo arreglo (TABLA2) con los números que sean
# menores que LIMITE. Además guarda en RES la cantidad de elementos guardados en el segundo arreglo. Completar instrucciones
# faltantes.

.data
TABLA1: .word 1, 12, 6, 3, 4, 5, 21, 11
CANT:   .word 8
LIMITE: .word 5
RES:    .word 0
TABLA2: .word 0

.text
        ld $t0, CANT($zero)
        ld $t1, LIMITE($zero)   #! completar
        dadd $t2, $zero, $zero
        dadd $t3, $zero, $zero
        dadd $t4, $zero, $zero  #! completar
LOOP:   ld $t5, TABLA1($t3)
        slt $t6, $t5, $t1
        beqz $t6, FUERA
        daddi $t2, $t2, 1
        sd $t5, TABLA2($t4)
        daddi $t4, $t4, 8
FUERA:  daddi $t3, $t3, 8
        daddi $t0, $t0, -1      #! completar
        bnez $t0, LOOP
        sd $t2, RES($zero)
        halt
.data
    V: .word 1,1,2,3,5,8,13
    R: .word 0
.text
    daddi $t0, $zero, 0 ; indice
    daddi $t2, $zero, 7 ; dimL
    daddi $t4, $zero, 1

    loop:   ld $t1, V($t0)      ; valor vec en pos $t0
            dadd $t5, $t5, $t1  ; sumo valor
            daddi $t0, $zero, 8 ; incremento indice
            dsub $t2, $t2, $t4  ; decremento dimL
            bnez $t2, loop      ; sig. posicion
    sd $t5, R($zero)            ; guardo valor en variable
halt
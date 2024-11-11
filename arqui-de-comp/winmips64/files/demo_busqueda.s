; Busqueda de elemento en vector
.data
    busca:  .word 7
    vect:   .word 1,4,8,10,7
    largo:  .word 5

.code
        dadd r10,r0,r0      ; registro R10 = 0
        dadd r1, r0, r0     ; registro R1 = indice
        ld r2, largo(r0)    ; calculamos la dimension del vector
        dsll r2, r2, 3      ; multiplico r2 * 8
        ld r3, busca(r0)    ; elemento buscado
loop:   ld r4, vect(r1)     ; elemento del vector a comparar
        beq r3, r4, found   ; salgo del loop si son iguales
        daddi r1, r1, 8     ; r1++ (8 byte)
        slt r5, r1, r2      ; comparo (resultado en r5)
        bnez r5, loop       ; continuo el ciclo?
        j end               ; el resultado buscado no se encontro
found:  daddi r10, r0, 1    ; coloco TRUE en r10
end:    halt                ; comando winmips de cierremint linux
#       Calcular atascos

.data
A:      .word   2,1,3
SUM:    .word   0

.text
        daddi $t2, $0, 0
        ld $t1, SUM($0)
        daddi $t3, $0, 3
loop:   ld $t4, A($t2)
        dadd $t1, $t1, $t4
        daddi $t3, $t3, -1
        bnez $t3, loop
        sd $t1, SUM($0)
        halt


# BTB desactivado:      RAW: 6  BTS: 0
# BTB activado:         RAW: 6  BTS: 2
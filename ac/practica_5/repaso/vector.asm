.data

V:      .word 2,4,8
R:      .word 0

.text

        daddi $t3, $0, 3
        daddi $t2, $0, 0
        daddi $t4, $0, 0
loop:   ld $t1, V($t2)
        dmul $t4, $t4, $t1
        daddi $t2, $t2, 8
        daddi $t3, $t3, -1
        bnez $t3, loop
        sd $t4, R($0)
        halt
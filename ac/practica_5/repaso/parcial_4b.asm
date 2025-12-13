.data
        datos:          .word   12, 42, 51, 23, 17, 37, -8, 16, 21, 33
        filtrados:      .word   0

.text
        daddi $sp, $0, 0x400 
        daddi $a0, $0, datos
        daddi $a1, $0, filtrados
        daddi $a2, $0, 5
        jal FILTRAR
halt

FILTRAR: daddi $sp, $sp, -32
        sd $ra, 0($sp)
        sd $s0, 8($sp)
        sd $s1, 16($sp)
        sd $s2, 24($sp)


        dadd $s0, $0, $a0       # datos
        dadd $s1, $0, $a1       # filtrados
        dadd $s2, $0, $a2       # contador

loop:   ld $a0, 0($s0)          # primer par
        jal EN_RANGO
        beqz $v0, otro


        ld $a0, 8($s0)          # segundo par
        jal EN_RANGO
        beqz $v0, otro

        ld $t0, 0($s0)
        ld $t1, 8($s0)

        sd $t0, 0($s1) 
        sd $t1, 8($s1) 
        daddi $s1,$s1, 16 
otro:   daddi $s0, $s0, 16
        daddi $s2, $s2, -1
        bnez $s2, loop


        ld $ra, 0($sp)
        ld $s0, 8($sp)
        ld $s1, 16($sp)
        ld $s2, 24($sp)
        daddi $sp, $sp, 32

        jr $ra 

EN_RANGO: dadd $v0, $0, $0
        slt $v0, $a0, $0        # a0<0 = 1  
        bnez $v0, fin
        slti $v0, $a0, 50       # a0<50 = 1
        beqz $v0, fin
        daddi $v0, $0, 1
fin:  jr $ra
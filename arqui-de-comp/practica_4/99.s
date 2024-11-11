.data
    string: .asciiz "Arquitectura de Computadoras"
    longitud: .word 0

.code
    dadd $t2, $0, $0
    dadd $t3, $0, $0
    loop: lbu $t1, string($t2)
    beqz $t1, fin
    daddi $t3, $t3, 1
    daddi $t2, $t2, 1
    j loop
fin: sd $t3, longitud($0)
halt
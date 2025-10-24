.data
    cadena:   .asciiz "Arquitectura de Computadoras"
    car:      .ascii "d"
    longitud: .word 0
    cant:     .word 0

.text
    ld $t1, longitud($0)
    lbu $t4, car($0)
    daddi $t0, $0, 1

loop: lbu $t2, cadena($t0) 
      daddi $t1, $t1, 1
      daddi $t0, $t0, 8
      bne $t2, $t4, noes
      daddi $t5, $t5, 1
noes: beqz $t2, loop
      sd $t1, longitud($0)
      sd $t5, cant($0)
halt
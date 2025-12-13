# Indicar cuantos atascos tipo RAW y BTS ejecutando el programa; BTB y Delay Slot deshabilitados

        .data
DATO:   .word   2

        .text
        daddi $t0, $0, 0
        ld $t1, DATO($0)
        daddi $t2, $0, 3
loop:   dadd $t0, $t0, $t1
        daddi $t2, $t2, -1
        bnez $t2, loop
        halt

# Con Forwarding deshabilitado: RAW: 7 BTS: 2 CPI: 26 / 13 = 2
# Con Forwarding habilitado:    RAW: 3 BTS: 2 CPI: 22 / 13 = 1.6
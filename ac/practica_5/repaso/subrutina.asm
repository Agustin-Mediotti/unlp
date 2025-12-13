#  Ejemplo de programa con llamada a muliple subrutina
#
                .data
cadena:         .asciiz "Caza"
                .text
                daddi $sp, $0, 0x400    # Tope de memoria de Datos
                daddi $a0, $0, cadena   # Primer argumento
                jal to_uppercase
                halt

# Cadena de Caracteres a Mayuscula
# Parametros:
#       - $a0 -> inicio de cadena
#
# Se utiliza la pila para guardar:
#       - $ra -> porque se invoca a otra subrutina
#       - $s0 -> para guardar la direccion de inicio de la cadena y correrla
to_uppercase:   daddi $sp, $sp, -16     # Reserva lugar en la pila -> 2 x 8
                sd $ra, 0($sp)
                sd $ra, 8($sp)
                dadd $s0, $a0, $zero    # Copia la direccion de inicio de la cadena
caseStrLoop:    lbu $a0, 0($s0)         # Recupera caracter actual y lo pone como argumento para upcase
                beq $a0, $zero, strFin  # Si es el fin de la cadena, termina
                jal upcase
                sb $v0, 0($s0)          # Guarda el caracter procesado en la cadena
                daddi $s0, $s0, 1       # Avanza al siguiente caracter
                j caseStrLoop
strFin:         ld $ra, 0($sp)
                ld $s0, 8($sp)
                daddi $sp, $sp, 16
                jr $ra

# Caracter a Mayúscula
# Parametros:
#       - $a0 -> caracter
#       - $v0 -> caracter en mayúscula
# No se utiliza la pila porque no se usan registros que deban ser salvados
upcase:         dadd $v0, $a0, $zero
                slti $t0, $v0, 0x61     # Compara con 'a' minúscula
                bnez $t0, salir         # No es un caracter en minúscula
                slti $t0, $v0, 0x7B     # Compara con el caracter siguiente a 'z' minúscula (z=7AH)
                beqz $t0, salir         # No es un caracter en minúscula
                daddi $v0, $v0, -0x20   # Pasa minúscula (pone a '0' el 6to bit)
salir:          jr $ra                  # Retorna al programa principal
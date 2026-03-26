# Archivos
# Archivos

## Aspectos físicos
- *Capacidad*: La memoria RAM tiene capacidad de acceso más limitada que, por ejemplo, un disco rígido.
- *Tiempo de Acceso*: La memoria RAM mide su tiempo de acceso en orden de nanosegundos, el disco rígido en milisegundos.

## Archivos -> Definición
Los datos que necesiten ser preservados mas alla del tiempo de ejecucución deben residir en archivos sobre dispositivos de almacenamiento permanente de información. Un archivo es una estructura de datos homgenea, caracterizada por el crecimiento y las modificaciones que se efectuan sobre estos.

## Archivos ->  Correspondencia archivo lógico - archivo físico
## Archivos -> Definición
Los datos que necesiten ser preservados mas alla del tiempo de ejecucución deben residir en archivos sobre dispositivos de almacenamiento permanente de información. Un archivo es una estructura de datos homgenea, caracterizada por el crecimiento y las modificaciones que se efectuan sobre estos.

## Archivos ->  Correspondencia archivo lógico - archivo físico
El SO opera sobre el archivo físico, el algoritmo utiliza un tipo de datos *file* que representa el nombre del mísmo. Se debe indicar que el archivo lógico corresponde al archivo físico administrado por el SO. Algunas definiciones de nombre físico contienen la extensión que se queire dar al archivo. Un archivo de tipo *string* contendrá elementos de *255 caracteres de longitud* en cada caso.

## Archivos -> Viaje de un Byte
Actores involucrados:
- Administrador de archivos: conjunto de programas del SO que tratan aspectos relacionados con archivos y dispositivos de E/S.
- Buffer E/S: agilizan la E/S de datos
- Procesador de E/S: 
- Controlador del disco: encargado de gestionar la operacion del disco.

## Archivos -> Tiipos de Archivo
**Registros:**
- Longitud predecible
    - Campos fijos o variables
- Longitud variable
    - Indicador de longitud
    - Segundo archivo
    - Delimitador
- Estudio de casos: ventajas y desventajas

**Claves:**
- Se concibe al registro como cantida de informaciónque lee o escribe
- Es conveniente identificar un registro con una llave o clave que se base en el contenido del mísmo.
    - **Unívoca/Primaria:** Identifican un elemento particular dentro de un archivo.
    - **Secundaria:** Puede no identificar a un único registro.

**Forma de acceso:**
- Serie
- Secuencial

## Archivos -> Claves
Forma canónica: Forma estándar para una llave, puede derivarse a partr de reglas bien definidas
Representación única para la llave, ajustada a la regla.

**Estudio de performance:**
- Punto de partida para futuras evaluaciones
- Costo: acceso a disco, num de comparaciones
- Caso promedio

## Archivos -> Eliminación
- Baja lógica
- Baja física

Diferencias, ventajas y desventajas.
- En la baja fisica el espacio se libera y puede volver a ser ocupado.
- En la baja lógica el espacio no se recupera pero el elemento puede ser recuperado.

## Archivos -> Buffer de Datos
Debido a que las operaciones de lectura/escritura sobre disco tienen una velocidad de acceso menor a la RAM (milisec vs nanosec o 10⁻³ vs 10⁻⁹), por una cuestion de performance se utiliza una memoria intermedia denonimada *buffer* que se encuentra en la memoria RAM. Donde las operaciones de *read* y *write* van a intentar acceder para leer o escribir en primera instancia al buffer y si este esta lleno (para la escritura) o vacio (para la lectura) entonces hara el SO hara una operación de *input*. 


## Archivos -> Operaciones esenciales sobre archivos
## Archivos -> Operaciones esenciales sobre archivos
- *Alta*: Ingresar nuevos datos al archivo.
- *Modificacion*: Alterar el contenido de algún dato del archivo.
- *Consulta*: Presentar el contenido total o parcial del archivo.
- *Baja*: Quitar información del archivo.

## Archivos -> Corte de Control
El proceso mediante el cuál la información de un archivo es procesada y presentada de forma organizada de acuerdo a la estructura que tiene un archivo.

## Archivos -> Actualizacion Maestro Detalle
Los elementos del detalle **SI O SI** son parte del maestro, por lo tanto podemos leer el maestro sin la necesidad de preguntar si el índice del maestro es **EOF**


## Archivos -> Merge
Involucra archivos con contenido similar, el cuál debe resumirse en un único archivo.
**Condiciones:**
    - Todos los archivos tienen la mísma estructura.
    - Todos están ordenados por igual criterio.
